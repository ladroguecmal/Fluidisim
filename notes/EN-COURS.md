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

Session : S297 — porte B, lot 3 : couplage B/W dans la référence 3D et aperçu animé.
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : « Continue, et j'aimerais pouvoir voir après », 2026-09-19.
Objectif : étendre les équations reçues ADR-149/152/153/164/165/166 aux deux dimensions
horizontales, puis montrer un calcul véritable de la référence par des images locales de banc.
Critères avant code : fond nul identique au bit au pas mobile S296 ; à ny=1 cas S253 contre
HOS (profil <2 %, harmonique <20 % à 128 colonnes, décroissants) ; fond uniforme traversant
sans perturbation créée ; invariance transverse à l'arrondi ; source réellement 3D et refus
atomiques sans allocation. Toute réception manquée reste publiée, sans déplacer ses seuils.
Aperçu : surfaces rendues directement en images, sans sérialisation d'état δ (I-17), commande
reproductible et paramètres publiés. Aucune réception perceptive ou temps réel anticipée.

### Plan

- [x] **P1** — état réel, jeton et plan seuls ; copie unique, branche B archivée, diff vide.
- [x] **P2** — contrats de fond 3D, réserves, géométrie et fantômes du total ; affinage homogène.
- [x] **P3** — advection croisée/source, bandes aux quatre bords, éponge et pas atomique ; tests
  fond nul, courant traversant, refus et allocations.
- [ ] **P4** — réception : cas limite HOS, invariance transverse et cas oblique ; preuve S297.
- [ ] **P5** — aperçu animé local calculé depuis le pas 3D, rendu en images de banc, vérification
  visuelle et livraison ; aucune page HTML ni état δ écrit sur disque.
- [ ] **P6** — rituel REPRISE §6, file et feuille de route, jeton libre, commits vérifiés.

### Notes de reprise

La 2D reste le témoin. `delta_coupling.rs` porte les équations et les bandes ADR-166.
Le mode mobile 3D S296 est dans `delta3d_mobile.rs` ; lui garder ses bits au fond nul.
La production GPU et le raccord à l'afficheur de mer restent distincts de l'aperçu CPU.
Découper avant 15 minutes toute étape qui se prolonge.
