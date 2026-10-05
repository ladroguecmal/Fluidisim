# -*- coding: utf-8 -*-
"""Le banc de non-régression — S483 (ADR-222 D3) : ce qui change sans qu'on le veuille se voit à la session qui le change.

Trois vérifications, moins de trois minutes, contre `docs/validation/EMPREINTES.md` (versionné) :

1. **La référence au bit** — `apic3d_bulle` (la bulle de S479, 4 pas) : son empreinte (positions, vitesses, `φ`) égale à celle
   inscrite, et la même avec 1 et 16 fils (ADR-222 D2 : le nombre de fils change la vitesse, jamais le résultat).
2. **La carte contre la référence** — `water-viewer --apic3d-poches CAS=plusieurs PAS=2` : aucune maille de poche différente à
   étiquettes égales, le même nombre de poches, volume et pression à 10⁻⁴.
3. **La scène `--v1` sur la carte** — 5 s : masse exacte au quantum, la trajectoire inscrite (pas, particules, colonnes en bande à
   5 s) ; le pas médian sous 1,3 fois celui inscrit.

    python outils/non_regression.py            # les trois ; code 1 si l'une échoue
    python outils/non_regression.py --inscrire # réinscrit les empreintes (un changement voulu, dit dans le journal)

Construit d'abord les binaires (`cargo build`, incrémental). `rituel.py fin` l'appelle.
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EMPREINTES = ROOT / "docs/validation/EMPREINTES.md"
BULLE = ROOT / "code/target/release/examples/apic3d_bulle.exe"
VIEWER = ROOT / "viewer/target/release/water-viewer.exe"
ENTETE = """# Empreintes de non-régression

*Tenu par `python outils/non_regression.py --inscrire` (S483, ADR-222 D3) — ne pas modifier à la main.* Une empreinte se réinscrit
quand un changement voulu modifie le résultat, et le journal de la session le dit. Le pas médian dépend de la carte et de sa
température : le seuil est 1,3 fois la valeur inscrite.

## Reproduire

`python outils/non_regression.py` (vérifier) ; `python outils/non_regression.py --inscrire` (réinscrire). Le détail des trois vérifications
est en tête de l'outil et dans [REFERENCE-PARALLELE-S483](REFERENCE-PARALLELE-S483.md) §3.

| clé | valeur |
|---|---|
"""


def run(args, env=None, cwd=ROOT, timeout=900):
    import os
    e = dict(os.environ, **(env or {}))
    p = subprocess.run(args, cwd=cwd, env=e, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=timeout)
    return p.returncode, p.stdout + p.stderr


def construire():
    for args, cwd in ((["cargo", "build", "--manifest-path", "code/Cargo.toml", "-p", "water-core", "--release", "--offline",
                        "--example", "apic3d_bulle"], ROOT),
                      (["cargo", "build", "--release", "--offline"], ROOT / "viewer")):
        code, out = run(args, cwd=cwd, timeout=1800)
        if code != 0:
            raise SystemExit("construction impossible :\n" + out[-2000:])


def mesurer() -> dict:
    m = {}
    for fils in ("1", "16"):
        _, out = run([str(BULLE), "4", "0.002", "500"], env={"FILS": fils})
        e = re.search(r"BULLE_EMPREINTE pas=(\d+) empreinte=([0-9a-f]+)", out)
        m[f"bulle_empreinte_fils_{fils}"] = e.group(2) if e else "absente"
    _, out = run([str(VIEWER), "--apic3d-poches"], env={"CAS": "plusieurs", "PAS": "2"})
    b = re.search(r"bilan detection .*dont_a_etiquettes_egales=(\d+) .*nombres_differents=(\d+) pire_ecart_V=(\S+) pire_ecart_P=(\S+)", out)
    if b:
        m["poches_ecarts_detection"] = b.group(1)
        m["poches_nombres_differents"] = b.group(2)
        m["poches_pire_ecart"] = f"{max(float(b.group(3)), float(b.group(4))):.1e}"
    else:
        m["poches_ecarts_detection"] = "absent"
    _, out = run([str(VIEWER), "--v1-banc"], env={"DUREE": "5", "SORTIE": "captures/non_regression"})
    t = re.search(r"V1_S458 t_s=5\.\d+ pas=(\d+) masse_ecart_quanta=(-?\d+) phi_fini=(\w+) colonnes_en_bande=(\d+) n=(\d+)", out)
    c = re.search(r"pas_mur_ms_mediane=([\d.]+)", out)
    m["v1_trajectoire_5s"] = f"pas={t.group(1)} colonnes={t.group(4)} n={t.group(5)}" if t else "absente"
    m["v1_masse_ecart_quanta"] = t.group(2) if t else "absent"
    m["v1_pas_median_ms"] = c.group(1) if c else "absent"
    return m


def lire() -> dict:
    if not EMPREINTES.exists():
        return {}
    return {a.strip("` "): b.strip("` ") for a, b in re.findall(r"^\| (`[^`]+`) \| (`[^`]*`) \|$", EMPREINTES.read_text(encoding="utf-8"), re.M)}


def verifier(m: dict, ref: dict) -> list:
    echecs = []
    if m["bulle_empreinte_fils_1"] != m["bulle_empreinte_fils_16"]:
        echecs.append(f"la référence dépend du nombre de fils : {m['bulle_empreinte_fils_1']} contre {m['bulle_empreinte_fils_16']}")
    for k in ("bulle_empreinte_fils_1", "v1_trajectoire_5s"):
        if ref.get(k) and m.get(k) != ref[k]:
            echecs.append(f"{k} : {m.get(k)} au lieu de {ref[k]}")
    if m.get("poches_ecarts_detection") != "0" or m.get("poches_nombres_differents") != "0":
        echecs.append(f"poches : {m.get('poches_ecarts_detection')} mailles différentes, {m.get('poches_nombres_differents')} nombres différents")
    elif float(m["poches_pire_ecart"]) > 1e-4:
        echecs.append(f"poches : écart {m['poches_pire_ecart']} > 1e-4")
    if m.get("v1_masse_ecart_quanta") != "0":
        echecs.append(f"--v1 : masse {m.get('v1_masse_ecart_quanta')} quanta")
    if ref.get("v1_pas_median_ms") and m.get("v1_pas_median_ms", "absent") != "absent":
        if float(m["v1_pas_median_ms"]) > 1.3 * float(ref["v1_pas_median_ms"]):
            echecs.append(f"--v1 : pas médian {m['v1_pas_median_ms']} ms > 1,3 × {ref['v1_pas_median_ms']}")
    return echecs


def main(argv) -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    construire()
    m = mesurer()
    if "--inscrire" in argv:
        lignes = "".join(f"| `{k}` | `{v}` |\n" for k, v in m.items())
        EMPREINTES.write_text(ENTETE + lignes, encoding="utf-8", newline="\n")
        print("NON_REGRESSION inscrite : " + ", ".join(f"{k}={v}" for k, v in m.items()))
        return 0
    echecs = verifier(m, lire())
    for e in echecs:
        print("ECHEC " + e)
    print("NON_REGRESSION " + ("échec" if echecs else "tenue") + " : " + ", ".join(f"{k}={v}" for k, v in m.items()))
    return 1 if echecs else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
