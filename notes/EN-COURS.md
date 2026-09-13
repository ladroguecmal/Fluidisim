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

Session : S211 — terminée
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : premier hôte GPU B/W de J1 après accord des sources.

### État réel et accord

master et trois copies propres à de63a00 ; jeton libre. Accord « oui » sur les 254 sources
crates.io S210, cache Cargo local et verrou versionné. Aucun vendoring.
Compteur 2 : file J1, couches B/W, construction effective.

### Plan

- [x] **P1** — accord et plan seul, jeton occupé.
- [x] **P2** — sources verrouillées, API et données GPU B/W depuis le cœur ; réception CPU.
- [x] **P3** — fenêtre, pipeline GPU, caméra interactive et B+impact ; compilation.
- [x] **P4** — comparaison GPU/CPU, capture locale, mesures distinctes CPU/GPU et contrôles ciblés.
- [x] **P5** — rituel §6, lancement, file active, passation, jeton libre, copies synchronisées.

### Notes de reprise

Critères avant mesures : B issu du même spectre que la bibliothèque, phases repliées,
aucun temps absolu f32 envoyé au GPU. Impact RadialTable à lambda/16.
Comparaison hauteur GPU/CPU sur scène S201+S203 : tolérance 3 mm (marche S201),
max/RMS publiés, pente mesurée sans réception physique par image.
Horodatage GPU si disponible, sinon indisponible explicite. Aucun ajout hors verrou.
J1 reste ouvert si le sillage ou une réception manque.
P2 : sources verrouillées récupérées. Background::render_components publie amplitude, kx/ky et phase
repliée relative à une origine monde ; refus atomique, stockage hôte, aucune allocation.
Test ciblé reçu aux temps 15 s, 1e6 s et u64::MAX, deux origines, refus domaine/capacité.

P3 : hôte winit/wgpu compilé hors réseau. API wgpu30 adaptée (CurrentSurfaceTexture,
Queue::present, InstanceDescriptor explicite). Grille projetée 2 px, shader partagé compute/rendu.
Modes --verify et --smoke ; Espace pause, R relance, B témoin, flèches déplacement, clic droit rotation.

P4 : DX12 reçu ; découverte multibackend arrêt natif 0xc0000005, cause non isolée.
Hauteur max 0,077657 mm ; GPU eau 960×540 médiane 0,048576 ms, pas de cadence complète reçue.
349 tests réussis/cinq ignorés ; fenêtre inspectée et fermée normalement. Voir HOTE-GPU-S211.

P5 : journal, index, README, feuille de route, file plurielle et angles actualisés.
A250 close ; A247 partielle. S212 porte W/sillage, J1 reste partiel. Compteur0.
Jeton libre ; avance rapide des trois copies après commit final, aucune copie créée.
