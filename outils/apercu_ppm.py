#!/usr/bin/env python3
"""Aperçu PNG d'une image de banc PPM — Python standard, sans réseau, sans dépendance.

ADR-124 autorise les images locales de banc « PPM et preview ». Les PPM que le dépôt écrit ne
s'ouvrent pas dans la plupart des visionneuses ; cet outil en fait un PNG **à côté**, sans toucher
à l'original et sans rien publier.

    python outils/apercu_ppm.py captures/s248/mer.ppm [sortie.png]

Il ne lit que du P6 binaire à 255 niveaux, le seul format que le dépôt écrit.
"""
import struct
import sys
import zlib
from pathlib import Path


def lire_ppm(chemin: Path) -> tuple[int, int, bytes]:
    data = chemin.read_bytes()
    champs: list[bytes] = []
    i = 0
    while len(champs) < 4:
        while i < len(data) and data[i : i + 1].isspace():
            i += 1
        if data[i : i + 1] == b"#":  # commentaire, jusqu'à la fin de ligne
            while i < len(data) and data[i : i + 1] not in (b"\n", b"\r"):
                i += 1
            continue
        debut = i
        while i < len(data) and not data[i : i + 1].isspace():
            i += 1
        champs.append(data[debut:i])
    if champs[0] != b"P6":
        raise SystemExit(f"{chemin} : seul le P6 binaire est lu, trouvé {champs[0]!r}")
    if champs[3] != b"255":
        raise SystemExit(f"{chemin} : seuls les 255 niveaux sont lus, trouvé {champs[3]!r}")
    largeur, hauteur = int(champs[1]), int(champs[2])
    pixels = data[i + 1 :]
    attendu = largeur * hauteur * 3
    if len(pixels) < attendu:
        raise SystemExit(f"{chemin} : {len(pixels)} octets pour {attendu} attendus")
    return largeur, hauteur, pixels[:attendu]


def morceau(nom: bytes, corps: bytes) -> bytes:
    return (
        struct.pack(">I", len(corps))
        + nom
        + corps
        + struct.pack(">I", zlib.crc32(nom + corps) & 0xFFFFFFFF)
    )


def ecrire_png(chemin: Path, largeur: int, hauteur: int, pixels: bytes) -> None:
    lignes = bytearray()
    pas = largeur * 3
    for j in range(hauteur):
        lignes.append(0)  # filtre « aucun » : l'aperçu n'a pas à être compact
        lignes += pixels[j * pas : (j + 1) * pas]
    chemin.write_bytes(
        b"\x89PNG\r\n\x1a\n"
        + morceau(b"IHDR", struct.pack(">IIBBBBB", largeur, hauteur, 8, 2, 0, 0, 0))
        + morceau(b"IDAT", zlib.compress(bytes(lignes), 9))
        + morceau(b"IEND", b"")
    )


def main(argv: list[str]) -> int:
    if len(argv) < 2:
        print(__doc__)
        return 2
    source = Path(argv[1])
    cible = Path(argv[2]) if len(argv) > 2 else source.with_suffix(".png")
    largeur, hauteur, pixels = lire_ppm(source)
    ecrire_png(cible, largeur, hauteur, pixels)
    print(f"{source} -> {cible} ({largeur}x{hauteur})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
