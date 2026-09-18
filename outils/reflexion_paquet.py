"""S269 : réception différentielle des douze traces du banc delta_reflection.
Usage : python outils/reflexion_paquet.py <dossier_des_logs>
Aucune dépendance externe ; voir REFLEXION-PAQUET-S269.md pour la portée.
"""
import math
from pathlib import Path
import re
import sys


def read(root, case, resolution):
    path = root / f"fluidisim-s269-{case}-{resolution}.log"
    text = path.read_text(encoding="utf-8-sig")
    if "MESURE cas=" not in text or "REFUS cas=" in text:
        raise ValueError(f"calcul incomplet ou refusé : {path}")
    rows = [(round(float(t) * 1000), float(y)) for t, y in
            re.findall(r"TRACE t=([\d.]+) eta=([-\d.]+)", text)]
    if [t for t, _ in rows] != list(range(5, 36001, 5)):
        raise ValueError(f"trace incomplète : {path}")
    values = [y for _, y in rows]
    if not all(math.isfinite(y) for y in values):
        raise ValueError(f"trace non finie : {path}")
    return values


def energy(values, begin, end):
    return math.fsum(y*y*0.005 for i, y in enumerate(values)
                     if begin <= (i+1)*5 <= end)


def compare(root):
    failures, reflections = [], []
    for res in ["025", "0125"]:
        names = ["garde", "garde-longue", "garde-eponge", "garde-longue-eponge", "mur", "eponge"]
        fields = {name: read(root, name, res) for name in names}
        for name, reference in [("garde-longue", "garde"),
                                ("garde-longue-eponge", "garde-eponge"),
                                ("mur", "garde"), ("eponge", "garde-eponge")]:
            guard = fields[reference]
            incident = energy(guard, 0, 12000)
            if incident <= 0:
                raise ValueError("énergie incidente nulle")
            delta = [a-b for a, b in zip(fields[name], guard)]
            before = math.sqrt(energy(delta, 0, 12000)/incident)
            reflection = math.sqrt(energy(delta, 14000, 36000)/incident)
            print(f"DIFFERENCE dx={res} cas={name} temoin={reference} incident={before:.9g} retour={reflection:.9g}")
            if before > 0.001:
                failures.append(f"{res}/{name}: incident différent")
            if name.startswith("garde-longue") and reflection > 0.001:
                failures.append(f"{res}/{name}: garde différentielle contaminée")
            if name == "mur" and reflection < 0.5:
                failures.append(f"{res}: mur non discriminant")
            if name == "eponge":
                reflections.append(reflection)
                if reflection > 0.01:
                    failures.append(f"{res}: réflexion différentielle >1 %")
    difference = abs(reflections[0]-reflections[1])
    print(f"CONVERGENCE ecart_absolu={difference:.9g} limite=0.002")
    if difference > 0.002:
        failures.append("réflexion non convergée")
    if failures:
        raise ValueError("; ".join(failures))
    print("RECU : effet du bord droit relatif au domaine long, sur ce paquet seulement")


if __name__ == "__main__":
    try:
        if len(sys.argv) != 2:
            raise ValueError("indiquer le dossier des douze logs")
        compare(Path(sys.argv[1]))
    except (ValueError, OSError) as error:
        print(f"NON_RECU : {error}", file=sys.stderr)
        sys.exit(1)
