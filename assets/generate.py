#!/usr/bin/env python3
"""Generates the rebootinto logo and icons.

The icon is described once, as a list of shapes in a 256x256 design space.
Each shape knows how to write itself out as SVG and how to measure its signed
distance, which the rasterizer uses to produce antialiased PNG and ICO files.
The script needs nothing beyond the Python standard library.

Run `python3 assets/generate.py` to regenerate everything under `assets/`.
"""

import math
import os
import struct
import zlib

ROOT = os.path.dirname(os.path.abspath(__file__))

DESIGN = 256.0

BG_TOP = (0x4F, 0x6B, 0xED)
BG_BOTTOM = (0x2B, 0x3A, 0xA8)
WHITE = (0xFF, 0xFF, 0xFF)
ACCENT = (0xFF, 0xC2, 0x33)

# At tiny sizes the three boot menu entries blur together, so only a thicker
# selected entry is drawn.
SMALL_MAX = 24

PNG_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]
ICO_SIZES = [16, 24, 32, 48, 64, 256]


def hex_color(c):
    return "#%02X%02X%02X" % c


def fmt(v):
    return ("%.2f" % v).rstrip("0").rstrip(".")


# --- Shapes ------------------------------------------------------------------
#
# Every shape provides:
# - `sdf(px, py)`: signed distance to the outline, negative inside;
# - `color(py)`: fill color, which may only vary vertically;
# - `opacity`;
# - `svg()`: the SVG elements drawing the same shape.


class GradientRoundRect:
    """A rounded rectangle filled with a top to bottom gradient."""

    GRADIENT_ID = "rebootinto-bg"

    def __init__(self, x0, y0, x1, y1, r, top, bottom):
        self.x0, self.y0, self.x1, self.y1, self.r = x0, y0, x1, y1, r
        self.top, self.bottom = top, bottom
        self.cx, self.cy = (x0 + x1) / 2, (y0 + y1) / 2
        self.bx, self.by = (x1 - x0) / 2, (y1 - y0) / 2
        self.opacity = 1.0

    def sdf(self, px, py):
        qx = abs(px - self.cx) - self.bx + self.r
        qy = abs(py - self.cy) - self.by + self.r
        outside = math.hypot(max(qx, 0.0), max(qy, 0.0))
        return outside + min(max(qx, qy), 0.0) - self.r

    def color(self, py):
        t = max(0.0, min(1.0, (py - self.y0) / (self.y1 - self.y0)))
        return tuple(self.top[k] + (self.bottom[k] - self.top[k]) * t for k in range(3))

    def svg(self):
        return [
            "<defs>",
            '  <linearGradient id="%s" x1="0" y1="%s" x2="0" y2="%s" gradientUnits="userSpaceOnUse">'
            % (self.GRADIENT_ID, fmt(self.y0), fmt(self.y1)),
            '    <stop offset="0" stop-color="%s"/>' % hex_color(self.top),
            '    <stop offset="1" stop-color="%s"/>' % hex_color(self.bottom),
            "  </linearGradient>",
            "</defs>",
            '<rect x="%s" y="%s" width="%s" height="%s" rx="%s" fill="url(#%s)"/>'
            % (
                fmt(self.x0),
                fmt(self.y0),
                fmt(self.x1 - self.x0),
                fmt(self.y1 - self.y0),
                fmt(self.r),
                self.GRADIENT_ID,
            ),
        ]


class Arc:
    """A round-capped ring segment going clockwise (screen coordinates, y down)."""

    def __init__(self, c, r, width, start_deg, end_deg, color):
        self.c, self.r, self.width = c, r, width
        self.start_deg, self.end_deg = start_deg, end_deg
        self.fill = color
        self.opacity = 1.0
        self.start = self.point(start_deg)
        self.end = self.point(end_deg)

    def point(self, deg):
        a = math.radians(deg)
        return (self.c[0] + self.r * math.cos(a), self.c[1] + self.r * math.sin(a))

    def sdf(self, px, py):
        dx, dy = px - self.c[0], py - self.c[1]
        deg = (math.degrees(math.atan2(dy, dx)) - self.start_deg) % 360.0 + self.start_deg
        if deg <= self.end_deg:
            return abs(math.hypot(dx, dy) - self.r) - self.width / 2
        return (
            min(
                math.hypot(px - self.start[0], py - self.start[1]),
                math.hypot(px - self.end[0], py - self.end[1]),
            )
            - self.width / 2
        )

    def color(self, _py):
        return self.fill

    def svg(self):
        large = 1 if (self.end_deg - self.start_deg) % 360 > 180 else 0
        return [
            '<path d="M%s %sA%s %s 0 %d 1 %s %s" fill="none" stroke="%s" stroke-width="%s" stroke-linecap="round"/>'
            % (
                fmt(self.start[0]),
                fmt(self.start[1]),
                fmt(self.r),
                fmt(self.r),
                large,
                fmt(self.end[0]),
                fmt(self.end[1]),
                hex_color(self.fill),
                fmt(self.width),
            )
        ]


class RoundPolygon:
    """A polygon whose outline is grown by `round`, which rounds its corners."""

    def __init__(self, points, round, color):
        self.points, self.round = points, round
        self.fill = color
        self.opacity = 1.0

    def sdf(self, px, py):
        pts = self.points
        d = (px - pts[0][0]) ** 2 + (py - pts[0][1]) ** 2
        s = 1.0
        j = len(pts) - 1
        for i in range(len(pts)):
            ex, ey = pts[j][0] - pts[i][0], pts[j][1] - pts[i][1]
            wx, wy = px - pts[i][0], py - pts[i][1]
            h = max(0.0, min(1.0, (wx * ex + wy * ey) / (ex * ex + ey * ey)))
            bx, by = wx - ex * h, wy - ey * h
            d = min(d, bx * bx + by * by)
            c1 = py >= pts[i][1]
            c2 = py < pts[j][1]
            c3 = ex * wy > ey * wx
            if (c1 and c2 and c3) or (not c1 and not c2 and not c3):
                s = -s
            j = i
        return s * math.sqrt(d) - self.round

    def color(self, _py):
        return self.fill

    def svg(self):
        points = " ".join("%s,%s" % (fmt(x), fmt(y)) for x, y in self.points)
        color = hex_color(self.fill)
        return [
            '<polygon points="%s" fill="%s" stroke="%s" stroke-width="%s" stroke-linejoin="round"/>'
            % (points, color, color, fmt(self.round * 2))
        ]


class Capsule:
    """A line segment with round caps."""

    def __init__(self, a, b, r, color, opacity=1.0):
        self.a, self.b, self.r = a, b, r
        self.fill = color
        self.opacity = opacity

    def sdf(self, px, py):
        pax, pay = px - self.a[0], py - self.a[1]
        bax, bay = self.b[0] - self.a[0], self.b[1] - self.a[1]
        h = max(0.0, min(1.0, (pax * bax + pay * bay) / (bax * bax + bay * bay)))
        return math.hypot(pax - bax * h, pay - bay * h) - self.r

    def color(self, _py):
        return self.fill

    def svg(self):
        extra = "" if self.opacity >= 1.0 else ' stroke-opacity="%s"' % fmt(self.opacity)
        return [
            '<line x1="%s" y1="%s" x2="%s" y2="%s" stroke="%s" stroke-width="%s" stroke-linecap="round"%s/>'
            % (
                fmt(self.a[0]),
                fmt(self.a[1]),
                fmt(self.b[0]),
                fmt(self.b[1]),
                hex_color(self.fill),
                fmt(self.r * 2),
                extra,
            )
        ]


def icon_shapes(small=False):
    """Returns the icon shapes, bottom to top."""
    shapes = [
        GradientRoundRect(8.0, 8.0, 248.0, 248.0, 48.0, BG_TOP, BG_BOTTOM),
        # Restart arc from the gap at the top right, around the bottom, up to
        # the top.
        Arc((128.0, 128.0), 80.0, 22.0, -40.0, 270.0, WHITE),
        # Arrowhead at the top end of the arc, pointing clockwise into the gap.
        RoundPolygon([(124.0, 20.0), (160.0, 48.0), (124.0, 76.0)], 3.0, WHITE),
    ]
    # Boot menu entries inside the ring; the middle one is the selected entry.
    if small:
        shapes.append(Capsule((100.0, 128.0), (156.0, 128.0), 12.0, ACCENT))
    else:
        shapes += [
            Capsule((93.0, 104.0), (163.0, 104.0), 7.0, WHITE, 0.55),
            Capsule((93.0, 128.0), (163.0, 128.0), 7.0, ACCENT),
            Capsule((93.0, 152.0), (163.0, 152.0), 7.0, WHITE, 0.55),
        ]
    return shapes


# --- Rasterization -----------------------------------------------------------


def render(size):
    scale = DESIGN / size
    shapes = icon_shapes(small=size <= SMALL_MAX)
    rows = []
    for j in range(size):
        row = bytearray()
        py = (j + 0.5) * scale
        colors = [shape.color(py) for shape in shapes]
        for i in range(size):
            px = (i + 0.5) * scale
            r = g = b = a = 0.0
            for shape, (cr, cg, cb) in zip(shapes, colors):
                cov = max(0.0, min(1.0, 0.5 - shape.sdf(px, py) / scale)) * shape.opacity
                if cov <= 0.0:
                    continue
                # Premultiplied "over" compositing.
                r = cr * cov + r * (1.0 - cov)
                g = cg * cov + g * (1.0 - cov)
                b = cb * cov + b * (1.0 - cov)
                a = cov + a * (1.0 - cov)
            if a > 0.0:
                pixel = (r / a, g / a, b / a, a * 255.0)
            else:
                pixel = (0.0, 0.0, 0.0, 0.0)
            row.extend(max(0, min(255, int(round(v)))) for v in pixel)
        rows.append(bytes(row))
    return rows


def encode_png(size, rows):
    def chunk(kind, data):
        body = kind + data
        return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)

    raw = b"".join(b"\x00" + row for row in rows)
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 6, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(raw, 9))
        + chunk(b"IEND", b"")
    )


def encode_ico(pngs):
    """Packs PNG images into an ICO container (supported since Windows Vista)."""
    header = struct.pack("<HHH", 0, 1, len(pngs))
    offset = len(header) + 16 * len(pngs)
    entries = b""
    for size, data in pngs:
        dim = 0 if size >= 256 else size
        entries += struct.pack("<BBBBHHII", dim, dim, 0, 0, 1, 32, len(data), offset)
        offset += len(data)
    return header + entries + b"".join(data for _, data in pngs)


# --- SVG ---------------------------------------------------------------------


def svg_icon_body():
    return "\n".join("  " + line for shape in icon_shapes() for line in shape.svg())


def svg_icon():
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">\n'
        "%s\n</svg>\n" % svg_icon_body()
    )


def svg_logo():
    return """<svg xmlns="http://www.w3.org/2000/svg" width="920" height="256" viewBox="0 0 920 256">
  <style>
    .reboot { fill: #1B2366; }
    .into { fill: %(into)s; }
    @media (prefers-color-scheme: dark) {
      .reboot { fill: #E8ECFF; }
      .into { fill: #8EA2FF; }
    }
  </style>
%(body)s
  <text x="288" y="164" font-family="Inter, 'Segoe UI', Ubuntu, Cantarell, 'Helvetica Neue', Arial, sans-serif" font-size="104" font-weight="700" textLength="608" lengthAdjust="spacingAndGlyphs"><tspan class="reboot">reboot</tspan><tspan class="into">into</tspan></text>
</svg>
""" % {"body": svg_icon_body(), "into": hex_color(BG_TOP)}


# --- Output ------------------------------------------------------------------


def write(path, data):
    path = os.path.join(ROOT, path)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    mode = "w" if isinstance(data, str) else "wb"
    with open(path, mode) as f:
        f.write(data)


def main():
    write("logo.svg", svg_logo())
    write("icons/hicolor/scalable/apps/rebootinto.svg", svg_icon())

    pngs = {}
    for size in PNG_SIZES:
        pngs[size] = encode_png(size, render(size))
        write("icons/hicolor/%dx%d/apps/rebootinto.png" % (size, size), pngs[size])

    write("icon.ico", encode_ico([(size, pngs[size]) for size in ICO_SIZES]))


if __name__ == "__main__":
    main()
