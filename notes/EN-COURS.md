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

Session : S289 — en cours
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : reprendre le projet ; suite A276 déclarée par S288.
Objectif : un solveur de pression **résident** GPU — réductions et cycle sans retour CPU par
itération — que le **pas réel** consomme sous les portes d'acceptation inchangées du cœur.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [x] **P2** — cœur : crochet d'un candidat de pression externe dans la projection mobile.
  Le candidat ne franchit aucune porte : le cœur recalcule le vrai résidu et garde ADR-143/144.
  Refus atomique d'un candidat non fini ou de mauvaise forme. Tests.
  *Découpage corrigé : l'export séparé du second membre et de la diagonale, déclaré comme étape
  distincte, n'existe pas — le crochet passe `rhs` au candidat, et l'inverse de la diagonale se
  déduit exactement des lignes déjà exportées. Une étape de moins, pas une de moins faite.*
- [x] **P3** — GPU : cycle PCG résident — réductions d'arbre, `α`/`β` produits et consommés sur
  la carte, aucun retour CPU entre itérations ; réception contre le CG du cœur.
- [ ] **P4** — consommation par le **pas réel** : candidat GPU proposé au pas mobile, itérations
  restantes, acceptations/refus et coût mesurés contre le chemin CPU seul.
- [ ] **P5** — rituel §6 : preuve, journal, registres/index/feuille, jeton libre.

### Notes de reprise

Suite déclarée par S288 : ne pas ouvrir une micro-optimisation isolée. La brique S288
(opérateur + Jacobi amorti, export des lignes au bit) est acquise ; ce qui manque est le
**cycle**, les **réductions** et le **consommateur**.

Thèse d'intégration : **le GPU propose, le cœur dispose.** Le candidat entre par le chemin
warm d'ADR-169 déjà existant — `project` recalcule `r = b − A·p` sur CPU, puis CG poursuit
sous les portes d'ADR-143 (plancher d'arrondi) et ADR-144 (tolérance physique de S199).
Aucune porte n'est déplacée vers le GPU, aucune publication partielle : un mauvais candidat
coûte des itérations, il ne peut pas faire accepter un pas faux. C'est ce qui rend la
consommation par le pas réel possible sans réception physique du GPU lui-même.

P2 : crochet reçu. `step_surface_mobile_with` + `project_with`, `None` reproduit le pas
historique **au bit** (témoin comparé sur u/w/p/η). Candidat exact → itérations strictement
inférieures au témoin ; candidat absurde (1e4·bruit) → toujours accepté par les portes, non
dégradé ; NaN et +∞ → refus atomique, départ restauré au bit ; `propose` négatif → pas
historique ; réserve de lignes mal dimensionnée → `Err(Shape)`, candidat jamais consulté,
rien publié. 40 pas d'affilée avec oracle parfait : dérive de surface <= 1e-6 m, départ chaud
d'ADR-169 effectivement vu par le candidat. Suite cœur/harnais : 508 réussis, 21 ignorés,
0 échec (les décomptes de S288 — 508/19 — ne se recoupent pas exactement ; le mien est mesuré
sur `cargo test` dans `code/`).

P3 : cycle reçu, `--pression-cg`. Réductions à zéro itération : `⟨r,z⟩₀` et `‖r₀‖²` du GPU
contre le CPU, écart relatif ≤ 2e-7 (critère 1e-5) ; pression ressortie au bit. Convergence
identique au miroir CPU du **même** cycle : rapport des vrais résidus (arbitrés en f64)
0,964 à 1,094, et 1,0000 à 8 et 32 itérations sur les trois tailles. Repos : second membre
nul → pression nulle exacte. RTX 5070 / DX12.

Coût, médiane sur 9 après un premier appel écarté, transferts et attente compris :
32 768 mailles, 128 itérations → 5,26–5,50 ms complet, 1,679–1,685 ms GPU seul, contre
69,2–74,4 ms du miroir CPU (×13 complet, ×41 sur la carte) ; 32 itérations → 2,06–2,19 ms
contre 17,6–18,4. 6 656 mailles, 128 itérations → 4,23–4,24 ms complet, 1,15 ms GPU seul,
contre 13,5–14,7 ms CPU (×3,2). **589 mailles : le GPU perd** — 3,81–3,85 ms contre 1,09 ms
CPU à 128 itérations ; l'appel porte 0,27–0,63 ms de frais fixes quoi qu'il calcule.
Résidu relatif vrai atteint : 1,65e-4 à 6 656 mailles, 2,64e-4 à 32 768, pour 128 itérations.

**Limite mesurée, à ne pas masquer : le cycle alloue.** 82 allocations à 0 itération, puis
≈ 7 par itération — 990 à 128 — dans l'encodage wgpu lui-même, pas dans notre empaquetage.
Le chemin d'image d'ADR-145 n'admet pas cela ; l'enregistrement du cycle une fois pour toutes
(paquet de commandes réutilisé, ou dispatch indirect) est le lot qui lèverait ce point.
Première mesure fautive corrigée avant publication : le compteur ne suivait que `propose`.

Ce qui reste hors de ce lot, et doit le rester tant qu'il n'est pas prouvé : identité
inter-GPU, budget eau de 2 ms reçu, I-06 du chemin d'image, multigrille GPU, 3D.
