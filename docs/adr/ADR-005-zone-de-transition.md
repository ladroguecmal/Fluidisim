

---

## Note corrective — B-S26 : le §2 est mesuré, et son réglage est remplacé

> *Reportée de la lignée B le 2026-09-06 (S35), renvois renumérotés. Une **seconde voie**, indépendante
> de celle-ci, desserre la même contrainte : voir [`ADR-043`](ADR-043-deux-lignees-ont-ecrit-le-meme-solveur.md) §4 — et son §5, qui distingue les
> **trois** fonctions que ce §2 appelle « éponge ».*

Le §2 de cet ADR n'avait jamais été confronté à du code. Il l'a été en B-S26, par le cas C05
([ADR-042](ADR-042-l-eponge-mesuree-et-la-borne-de-lambda-cut-rouverte.md)), et trois de ses
affirmations doivent être lues avec ce qui suit.

**1. Le réglage `σ_max ≈ 4·c/L_s` ne donne pas `R < 1 %`.** Avec le profil quadratique du §2,
`∫₀^{L_s} σ_max·(x/L_s)² dx = σ_max·L_s/3`, donc `R ≈ exp(−8/3) = 6,95 %`. **Mesuré : 7,0 %.**
L'arithmétique et la mesure concordent à 1,3 %. Il faudrait `σ_max ≈ 6,9·c/L_s` pour tenir la
promesse ; **ADR-042 D1 retient `10·c/L_s`**, qui donne `1,5·10⁻³` avec de la marge.

**2. La formule `R ≈ exp(−2∫σ/c ds)` est excellente, puis s'effondre.** Jusqu'à `10·c/L_s` elle est
juste à quelques pour cent. Au-delà elle promet `2·10⁻⁶` là où on mesure `8,8·10⁻⁴` : la mesure
**sature vers `9·10⁻⁴`**, un plancher qui est la **réflexion à l'entrée de l'éponge** — l'impédance
que le modèle ne contient pas. **Amortir plus fort que `≈10·c/L_s` ne sert à rien**, ce que la
formule monotone ne peut pas exprimer.

**3. La condition `L_s ≥ λ_δ/2` n'est pas exercée en eau peu profonde.** À `σ_max` proportionnel à
`c/L_s`, `R` ne bouge pas quand l'éponge passe de `λ` à `λ/8`. Ce qui borne l'éponge par le bas est
**`σ_max·dt < 1`**, soit `L_s ≳ K·CFL·dx ≈ 5 mailles` — la **maille et le pas de temps**, pas la
longueur d'onde. **ADR-042 D2** remplace la règle ; **D4** rouvre en conséquence la borne haute de
`λ_cut` que `DOSSIER-B2 §3.1` en tirait.

**Ce qui reste vrai dans ce §2, et qui compte** : le §2.1 — *`λ_δ` est borné par construction, donc
l'éponge est dimensionnée par `λ_cut` et non par la physique de la scène* — n'est pas touché par la
mesure. Il l'est par D2, mais dans le sens qui l'arrange : si la largeur ne dépend plus de `λ` du
tout, l'argument de §2.1 devient superflu plutôt que faux.

**Et une réserve qui empêche de conclure** : le solveur de B-S26 est **non dispersif**. Une éponge
d'eau profonde doit absorber une **bande** de célérités, et c'est peut-être exactement ce dont
`L_s ≥ λ/2` protégeait. Voir ADR-042 §6.
