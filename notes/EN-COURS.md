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

Session : S201 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : direction utilisateur, rendre B visible par un exemple CPU caméra/rayons
et PPM local sans dépendance. Image explicitement autorisée après question, le13/09.
Priorité sur S200-1, conservée dans la file ; pas de critère perceptuel inventé.

### Plan

- [x] **P1** — état réel, quatre copies9a25798 propres ; jeton/plan seuls.
- [x] **P2** — caméra, intersection de B par rayons, lumière de diagnostic et PPM ;
  utiliser le vrai Background::eval, tests géométriques et export.
- [x] **P3** — produire une vue lisible, inspection visuelle, témoin plat et second
  instant ; publier paramètres/coût sans prétendre recevoir la perception.
- [ ] **P4** — documenter direction/autorisation et prochaines décisions budget puis
  périmètre δ/V ; file/index/passation/journal, compteur et copies synchronisées.

### Notes de reprise

REPRISE et invariants déjà lus dans cette conversation, état identique à9a25798.
Seuil numérique2 % acquis depuis S190 ; le rendu permettra une appréciation humaine,
aucune image seule ne ferme B4/A50 ni A98. V attend la demande gameplay selon nouvelle
direction, le changement architectural détaillé devra être acté dans son ordre.

P2 : rayons évaluent B JONSWAP N32 directement. Premier jet256 itérations
laissait997/1021 rayons rasants en magenta ; plafond4096 supprime les refus
sans modifier la tolérance3mm, t12=0xa52ff81902b150c3. Premier export échoué
car captures absent ; création explicite du répertoire ajoutée.

P3 : trois images640x360 inspectées, aucun rayon non résolu après correction.
T12 a52ff81902b150c3, t13 1df02ffb7c202b32, plat dae2f2514cad0324.
Cinq tests exemple passent (deux propres/trois hôte), aucun test bibliothèque refait.
Preview PNG = transcodage fidèle PPM via Pillow, sans retouche ; code zéro dépendance.
