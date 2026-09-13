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

Session : S213 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : corriger le cadrage du coût (demande utilisateur), inscrire l'espace d'optimisation dans
la trajectoire, puis construire et mesurer le levier temporel du sillage dans le cœur.

### Entrée utilisateur (2026-09-13, après S212)

« Continue avec la piste CPU en S213, mais corrige le cadrage : c'est l'implémentation actuelle
qui dépasse le budget, pas le sillage ni l'objectif final qui sont refusés. » Intégrer à la
trajectoire LOD spatiaux, spectraux et temporels, visibilité, mutualisation ; ne pas attendre
l'échec de deux optimisations pour demander de réduire l'ambition ; chaque mesure précise
techniques présentes, absentes et domaine de validité ; 2 ms = objectif à éprouver sur leur
combinaison, sans présumer succès ni échec ; conserver A251 et la vérification de la composition
impact+sillage dans les travaux nécessaires.

### État réel

master et trois copies propres à 1da11b6 (S212 close 10:30). Maillons 0. Machine : AMD Ryzen AI 7
350, RTX 5070 Laptop (DX12). Aucune dépendance nouvelle prévue.

### Thèse et critères du levier temporel, déclarés avant toute mesure

Dans la solution de Duhamel (ADR-069), un tronçon achevé ne fait plus que tourner : `(η, v/ω)`
subit la rotation `R(ω·Δt)`. Par nœud, les tronçons achevés se replient donc en **un état à
l'instant de référence** (début du contexte), tourné par image d'une phase entière
`phase(fréquence, t − t_réf)` ; seuls les tronçons **en cours** s'évaluent par
`ModalPressure::sample`, sur des modes **préconstruits** à la construction. Résonance `Ω = ±ω` :
aucune décomposition en phaseurs du tronçon actif (mal conditionnée là où vit le sillage de
Kelvin) — la forme sinc du cœur est conservée. Chemin cosmétique (ADR-129 §3), pas `sample` au bit.

Réception contre `bound_pressure::Prepared::from_journal` + `render_components` : reconstruction
de η et des pentes à 1e-5 de la norme L1 des coefficients ; instants naissance, milieu de tronçon,
bornes exactes de tronçon (±1 µs), fin de forçage, fin de contexte, **retour arrière** ; deux
recettes ; refus atomiques (contexte, instant, capacité, origine) ; témoin vérifié.

Mesure : coût par image médiane/max, **pic aux bornes de tronçon**, construction, mémoire, à 4 096
et 16 384 nœuds, pendant et après forçage. **Aucun seuil de réussite** : 2 ms est l'objectif de la
combinaison (ADR-131), pas d'un levier seul. Prédiction écrite pour être contredite : quelques
dixièmes de ms à 4 096 nœuds pendant le forçage, dizaines de µs après. Chaque résultat publié avec
techniques présentes (temps), absentes (espace, LOD spatial/spectral/temporel, visibilité,
mutualisation) et domaine de validité (fixture S212, une source, machine, instants).

### Plan

- [x] **P1** — jeton, entrée utilisateur, thèse, critères et plan seuls.
- [x] **P2** — ADR-131 (clarification utilisateur) ; note datée ADR-127 D7 ; FEUILLE-DE-ROUTE : espace d'optimisation, protocole de mesure, travaux nécessaires de J1 (A251, composition impact+sillage), §4 corrigé.
- [x] **P3** — propagation du cadrage : HOTE-GPU-S212 (note corrective, techniques/domaine), file active, REPRISE, index, README, viewer/README ; A252, L286.
- [>] **P4** — cœur : `pressure_timeline` (modes préconstruits, repli des tronçons achevés, publication par image, refus atomiques) ; compilation, test minimal.
- [ ] **P5** — réception contre `from_journal` : instants déclarés, bornes, retour arrière, deux recettes, refus, témoin.
- [ ] **P6** — mesure du levier seul (exemple release, sans GPU) : par image, pic aux bornes, construction, mémoire ; techniques et domaine.
- [ ] **P7** — hôte : levier par image ; `--verify` contre `from_journal`, contrôles S212 conservés ; coûts ; réception TEMPS-SILLAGE-S213 ; suite complète.
- [ ] **P8** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

P2 : ADR-131 actée (D1 implémentation, D2 espace d'optimisation ouvert, D3 techniques présentes/absentes/domaine,
D4 2 ms sur la combinaison, D5 aucune réduction fondée sur des optimisations isolées, D6 validité
conservée : A251 et composition impact+sillage). Note datée ADR-127 D7. Feuille de route : S212
recadré, J1-bis (tableau des techniques et travaux nécessaires), §3, §4, §5.

P3 : HOTE-GPU-S212 (note corrective en tête, verdicts barrés et recadrés, bloc techniques/domaine :
âges 3,17–5,15 s pendant forçage), file active (suivi S213, lignes J1/A247), REPRISE (token, file,
§4 S212 ⚠), index, README, viewer/README. A252 (sévérité 2, traitée ADR-131), L286. Note A251 :
un LOD spectral réduit la durée/rayon honnêtes. Mémoire privée : feedback cadrage-cout-implementation.
