#!/usr/bin/env python3
"""Le rendu d'atelier de la côte qui déferle — S675 (la séance visuelle ; R42).

Relit `calculs/s675_cote.bin` (la mer de S667 sur la plage de S364, avec et sans déferlement ; l'essai ignoré
`record_the_breaking_coast_for_the_visual_session_s675`) et `calculs/s675_controle.csv`, puis rend, numpy et PIL :

- `<sortie>_dessus.gif` : la surface vue de dessus sur les 750 derniers mètres, avec déferlement (en haut) et sans (en bas, le témoin) ;
- `<sortie>_coupe.gif` : la coupe à `n` = 0 sur les 950 derniers mètres, la surface sur le fond, le niveau moyen ;
- `<sortie>_profils.png` : `Hrms`, le niveau moyen `η̄` et le courant de dérive `V` le long de la plage.

    python outils/rendu_cote.py calculs/s675_cote.bin calculs/s675_controle.csv calculs/s675
"""
import struct
import sys

import numpy as np
from PIL import Image, ImageDraw, ImageFont

try:
    POLICE = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 13)
except OSError:
    POLICE = ImageFont.load_default()
ImageDraw.ImageDraw.font = POLICE


def lire(chemin):
    d = open(chemin, 'rb').read()
    o = 0

    def u32():
        nonlocal o
        v = struct.unpack_from('<I', d, o)[0]
        o += 4
        return v

    def f64():
        nonlocal o
        v = struct.unpack_from('<d', d, o)[0]
        o += 8
        return v

    def tab(t, n):
        nonlocal o
        v = np.frombuffer(d, t, n, o)
        o += n * np.dtype(t).itemsize
        return v

    r = {'ns': u32(), 'pas': f64()}
    for nom in ('h', 'hrms_avec', 'hrms_sans', 'eta', 'v'):
        r[nom] = tab('<f8', r['ns'])
    r['images'], r['dt'] = u32(), f64()
    r['nsd'], r['nnd'] = u32(), u32()
    r['s0'], r['n0'], r['d'] = f64(), f64(), f64()
    r['dessus'] = []
    for _ in range(r['images']):
        r['dessus'].append([tab('<f4', r['nsd'] * r['nnd']).reshape(r['nsd'], r['nnd']) for _ in range(2)])
    r['nc'], r['sc0'] = u32(), f64()
    r['coupe'] = []
    for _ in range(r['images']):
        r['coupe'].append([tab('<f4', r['nc']) for _ in range(2)])
    assert o == len(d), (o, len(d))
    return r


def controler(r, chemin):
    lignes = open(chemin, encoding='utf-8').read().split()
    attendu = [float(x) for x in lignes[1].split(',')]
    relu = [r['hrms_avec'][-1], r['eta'][-1], float(np.abs(r['v']).max())]
    pire = max(abs(a / b - 1) for a, b in zip(relu, attendu))
    print(f"contrôle : Hrms au rivage {relu[0]:.4f} m, η̄ {100*relu[1]:.2f} cm, |V| au plus {relu[2]:.4f} m/s ; écart relatif au plus {pire:.1e}")
    assert pire < 1e-6
    return relu


def couleur(eta):
    # La mer : le creux bleu sombre, la crête claire ; écrêté à ±1,2 m.
    x = np.clip((eta + 1.2) / 2.4, 0, 1)[..., None]
    sombre, clair = np.array([16, 52, 102]), np.array([214, 238, 250])
    return (sombre + x * (clair - sombre)).astype(np.uint8)


def dessus(r, sortie):
    ech = 2
    l, hgt = r['nsd'] * ech, r['nnd'] * ech
    marge, titre = 18, 20
    images = []
    for k in range(r['images']):
        im = Image.new('RGB', (l + 2 * marge, 2 * (hgt + titre) + 3 * marge + 22), (245, 245, 240))
        dr = ImageDraw.Draw(im)
        for p, (nom, y0) in enumerate([("avec déferlement (Battjes–Janssen), niveau moyen et courant de dérive", marge),
                                        ("sans déferlement — le témoin : la même mer, levée sans borne", 2 * marge + hgt + titre)]):
            champ = r['dessus'][k][p].T  # n vertical, s horizontal
            rgb = couleur(champ[::-1])
            tuile = Image.fromarray(rgb, 'RGB').resize((l, hgt), Image.NEAREST)
            im.paste(tuile, (marge, y0 + titre))
            dr.text((marge, y0 + 2), nom, fill=(20, 20, 20))
        ybas = 2 * (hgt + titre) + 2 * marge + 2
        for s in range(3200, 3951, 100):
            x = marge + int((s - r['s0']) / r['d'] * ech)
            h = r['h'][int(round(s / r['pas']))] if s / r['pas'] < r['ns'] else r['h'][-1]
            dr.line([(x, ybas), (x, ybas + 4)], fill=(60, 60, 60))
            dr.text((x - 18, ybas + 4), f"{h:.0f} m", fill=(60, 60, 60))
        dr.text((l + marge - 300, 2), f"t = {k * r['dt']:.1f} s — le large à gauche, le rivage à droite", fill=(60, 60, 60))
        images.append(im)
    images[0].save(sortie + '_dessus.gif', save_all=True, append_images=images[1:], duration=200, loop=0)


def coupe(r, sortie):
    n_pts, ex = r['nc'], 2
    l = n_pts * ex
    zmin, zmax, px = -6.0, 3.0, 40
    hgt = int((zmax - zmin) * px)
    marge = 30
    s = r['sc0'] + np.arange(n_pts) * r['d']
    i_tab = np.clip(np.round(s / r['pas']).astype(int), 0, r['ns'] - 1)
    fond = -r['h'][i_tab]
    eta_m = r['eta'][i_tab]
    zy = lambda z: marge + int((zmax - z) * px)
    images = []
    for k in range(r['images']):
        im = Image.new('RGB', (l + 2 * marge, hgt + 2 * marge + 34), (236, 242, 248))
        dr = ImageDraw.Draw(im)
        avec, sans = r['coupe'][k]
        for i in range(n_pts):
            for e in range(ex):
                x = marge + i * ex + e
                dr.line([(x, zy(max(fond[i], zmin))), (x, zy(min(avec[i], zmax)))], fill=(40, 110, 170))
                dr.line([(x, zy(max(fond[i], zmin))), (x, hgt + marge)], fill=(205, 180, 130))
        dr.line([(marge, zy(0)), (marge + l, zy(0))], fill=(150, 150, 150))
        dr.line([(marge + i * ex, zy(eta_m[i])) for i in range(n_pts)], fill=(250, 210, 40), width=2)
        dr.line([(marge + i * ex, zy(float(np.clip(sans[i], zmin, zmax)))) for i in range(n_pts)], fill=(200, 40, 40), width=1)
        dr.text((marge, 4), f"coupe à n = 0, du large (20 m de fond, à gauche) au rivage (1 m, à droite) — 950 m ; la hauteur exagérée ({px} px par mètre, 2 px par 2 m en long) — t = {k * r['dt']:.1f} s",
                fill=(20, 20, 20))
        dr.text((marge, hgt + marge + 2), "bleu : la surface avec déferlement ; jaune : le niveau moyen (le creux, puis la remontée) ; gris : le repos",
                fill=(40, 40, 40))
        dr.text((marge, hgt + marge + 18), "rouge : la même mer sans déferlement (le témoin, levée sans borne) ; sable : le fond, coupé à 6 m", fill=(40, 40, 40))
        images.append(im)
    images[0].save(sortie + '_coupe.gif', save_all=True, append_images=images[1:], duration=200, loop=0)


def trace(dr, boite, xs, series, ymin, ymax, titre, unite):
    x0, y0, x1, y1 = boite
    dr.rectangle(boite, outline=(120, 120, 120))
    dr.text((x0 + 4, y0 + 2), titre, fill=(20, 20, 20))
    fx = lambda x: x0 + (x - xs[0]) / (xs[-1] - xs[0]) * (x1 - x0)
    fy = lambda y: y1 - (min(max(y, ymin), ymax) - ymin) / (ymax - ymin) * (y1 - y0)
    for v in np.linspace(ymin, ymax, 5):
        dr.text((x0 - 44, fy(v) - 7), f"{v:.2f}", fill=(80, 80, 80))
    dr.text((x0 - 44, y0 - 16), unite, fill=(80, 80, 80))
    if ymin < 0 < ymax:
        dr.line([(x0, fy(0)), (x1, fy(0))], fill=(190, 190, 190))
    for ys, coul, nom, dy in series:
        dr.line([(fx(x), fy(y)) for x, y in zip(xs, ys)], fill=coul, width=2)
        dr.text((x1 - 260, y0 + 4 + dy), nom, fill=coul)


def profils(r, sortie):
    i0 = int(2500 / r['pas'])
    s = np.arange(r['ns'])[i0:] * r['pas']
    l, hgt, marge = 900, 170, 60
    im = Image.new('RGB', (l + 2 * marge, 3 * (hgt + 40) + 50), (250, 250, 248))
    dr = ImageDraw.Draw(im)
    h = r['h'][i0:]
    trace(dr, (marge, 30, marge + l, 30 + hgt), s, [(r['hrms_avec'][i0:], (30, 90, 170), "Hrms avec déferlement", 0),
          (r['hrms_sans'][i0:], (200, 40, 40), "Hrms sans déferlement (témoin)", 16), (0.4 * h, (150, 150, 150), "0,4·h", 32)],
          0.0, 2.4, "la hauteur de la mer, Hrms", "m")
    trace(dr, (marge, 70 + hgt, marge + l, 70 + 2 * hgt), s, [(100 * r['eta'][i0:], (200, 150, 20), "η̄, le niveau moyen", 0)],
          -5.0, 15.0, "le niveau moyen : le creux, puis la remontée au rivage", "cm")
    trace(dr, (marge, 110 + 2 * hgt, marge + l, 110 + 3 * hgt), s, [(r['v'][i0:], (40, 140, 90), "V, le long de la côte", 0)],
          -0.12, 0.04, "le courant de dérive littorale (le signe : le sens de t̂ ; ici vers +x)", "m/s")
    for sv in range(2500, 3951, 250):
        x = marge + (sv - s[0]) / (s[-1] - s[0]) * l
        hv = r['h'][min(int(sv / r['pas']), r['ns'] - 1)]
        dr.text((x - 30, 112 + 3 * hgt), f"{sv} m ({hv:.0f} m)", fill=(60, 60, 60))
    dr.text((marge, 8), "la mer de S667 (Hs 2,2 m, 7 à 12 s, −20° à +20°) sur une plage 1:50 — s le long de la normale (la profondeur entre parenthèses)",
            fill=(20, 20, 20))
    im.save(sortie + '_profils.png')


def main():
    r = lire(sys.argv[1])
    controler(r, sys.argv[2])
    dessus(r, sys.argv[3])
    coupe(r, sys.argv[3])
    profils(r, sys.argv[3])
    print("écrit :", sys.argv[3] + '_dessus.gif', sys.argv[3] + '_coupe.gif', sys.argv[3] + '_profils.png')


if __name__ == '__main__':
    main()
