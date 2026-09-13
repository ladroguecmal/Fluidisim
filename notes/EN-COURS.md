# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S212 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : file J1, couche W — sillage issu du cœur dans l'hôte GPU, comparé au cœur, coût mesuré.

### État réel

master et trois copies propres à 0e204ad (S211 close 10:04, jeton libre). Maillons 0 ; la suite
S211 nomme une ligne de la file (J1/W), pas un reliquat. Aucune dépendance nouvelle prévue.

### Thèse et critères, déclarés avant toute mesure

Le sillage du cœur est une somme modale (`spectral_pressure::Field`) : ADR-107 chiffre 25,6 ms
de préparation CPU pour 8 192 nœuds. Il n'a pas la forme de B (32 composantes). **Hypothèse à
éprouver, pas à croire** : publié en coefficients rebasés `[A, B, kx, ky]`, il se rend par somme
par sommet ; le coût dira si ce chemin tient ou s'il faut un autre chemin d'image (grille/texture,
transformée), à la manière d'ADR-129 pour l'impact.

Publication : `η(q) = Σ A cos(k·q) − B sin(k·q)`, pente `−k (A sin + B cos)`, où `(A + iB)` est
la réponse pondérée tournée de la phase repliée `k·origine` (PhaseQ32, aucun atan2, aucun temps
absolu au GPU — I-08). Emprise du champ appliquée au GPU comme au cœur.

Fixture sillage J1 (Froude de S156, lois de domaine transportées par similitude — **à vérifier**) :
σ 2 m, coupure 3 rad/m (σk 6), recette 64×128 (4 096 nœuds du demi-spectre) ; huit tronçons de
2 s à [3, 0] m/s sous 19 620 N (≈ 2 t), départ (−24, 4) m à la naissance de l'impact ; contexte
de 40 s ; emprise [−64, −48]–[64, 56] m ; repère/cellule 0, milieu 9,81 / 1025.

Critères : hauteur GPU contre cœur (B `eval` + impact direct + pression `sample_batch`) ≤ 3 mm
aux sondes, âges sillage 0/4/8/16/24/39 s ; témoin de résolution 64×128 contre 128×256 publié
(pas de seuil inventé, écart max rapporté à l'amplitude max) ; couture au bord de l'emprise
publiée ; admission `bound_pressure::Prepared::sample_world_batch` à `BREAKING_SLOPE` rapportée,
refus publié et non contourné ; coûts séparés CPU (préparation + publication) et GPU (passe
d'eau) en 640×360 et 960×540, deux recettes. Toute incompatibilité avec 2 ms est publiée ; une
issue technique va en ADR, une incompatibilité sans issue technique va à l'utilisateur (ADR-127 D7).

### Plan

- [x] **P1** — jeton, thèse, critères et plan seuls.
- [x] **P2** — cœur : `bound_pressure::Prepared::render_components` rebasé, refus atomiques, test contre `sample_batch`.
- [x] **P3** — hôte : fixture sillage, préparation par image, buffer GPU, somme modale bornée à l'emprise ; R/B/Home ; compilation.
- [x] **P4** — `--verify` : GPU contre cœur, témoin de résolution, couture, admission, coûts deux recettes, captures, fenêtre.
- [x] **P5** — réception HOTE-GPU-S212, décision chiffrée du chemin d'image du sillage, suite complète des tests.
- [x] **P6** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

P2 : `Field::render_components` / `bound_pressure::Prepared::render_components` + `component_count`.
Test contre `sample_batch` à 1e-5 relatif (norme L1 des coefficients), trois origines, quatre
décalages ; refus Context/Time/Capacity/origine sans écriture. Témoin : signe de B inversé → échec.

P3 : un seul `Wake` de huit tronçons suffit — un tronçon futur rend une réponse nulle
(`ModalPressure::sample`), l'émetteur progressif n'est pas requis pour un trajet scripté.
Journal de pression (époque 1), `Prepared::from_journal` + `render_components` par image ;
uniforme 128 octets, buffer sillage 16 384 × 16 octets (binding 3). R/B/Home inchangés : même
naissance que l'impact, B masque impact et sillage. Captures `captures/s212/`. Compilé release.
Piège : `R` est un alias PowerShell (Invoke-History) — une fonction nommée R n'écrit rien.

P4, RTX5070 Laptop/DX12, `--verify` (6 988 sondes, dont 424 de couture) :
hauteur GPU/cœur max 0,000089370 m (âge 40,01), 0,0000707–0,0000773 m sillage actif ; pentes ≤ 1,24e-4.
Témoin 64×128 / 128×256, amplitude max du fin : 4 s 0,567 mm/154,3 mm ; 8 s 1,355/117,0 ;
16 s 2,180/132,8 ; 24 s 4,324/45,4 ; 39 s 8,768/31,5 (0,37 %, 1,2 %, 1,6 %, 9,5 %, 28 %).
Couture (|η| au bord intérieur, 64×128 / fin) : 0,53/0,18 ; 1,37/0,59 ; 2,28/2,00 ; 5,62/3,35 ;
12,70/14,48 mm — contenu physique au bord, pas seulement récurrence. Enveloppe 0,135–0,165 ;
admission B+pression à BREAKING_SLOPE Ok aux cinq âges (impact non inclus dans ce contrôle).
Coûts médiane/max (ms) — 4 096 : CPU sillage 10,851/27,313 (640) 10,577/14,328 (960) ; GPU eau
1,902/1,913 (640) 4,103/4,143 (960). 16 384 : CPU 38,376/46,308 (640) 41,504/62,042 (960) ;
GPU 7,770/15,852 (640) 16,946/27,973 (960). Sans sillage S211 : GPU 0,018 / 0,049.
⇒ 7,6–7,9 ps par sommet×composante, 317–331 ns par nœud×tronçon CPU.
Captures 8 s scène/témoin : 60 182 pixels différents, max 28 niveaux, lignes 161–359 ; image de
différence : anneaux d'impact + motif de Kelvin derrière la source. `--smoke` : 120 images, code 0.

P5 : HOTE-GPU-S212 et viewer/README. Verdict : exact, refusé en coût ; incompatibilité mesurée
mais **pas encore un arbitrage** — deux leviers techniques non mesurés (temps dans le cœur :
états tournés + tronçon actif préconstruit ; espace dans l'hôte : grille cartésienne + transformée,
une somme polaire par texel ne gagne rien). Aucun ADR : le levier positif n'est pas mesuré.
Suite complète `code/` : 350 réussis (252+4+1+93), 5 ignorés (2+3), aucun échec.
A251 à ouvrir : durée honnête de recette et couture d'emprise du sillage visible non gardées.
Suite S213 : levier temporel dans le cœur (compteur 0, W avancée par P2).

P6 : journal S212, A251 + suivi A247, L285 (L248 couvrait déjà l'accord sur couche partagée),
index, README, REPRISE §4 et note sur le paragraphe S208 dépassé, feuille de route J1/§4, file
plurielle entière relue (A247, A249, J1, λ_cut/B2 datées S212 ; autres conservées). Invariants
I-03/I-04/I-06/I-08/I-15 relus, aucun devenu faux. Jeton libre ; copies avancées après ce commit.
