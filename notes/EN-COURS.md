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

Session : S298 — en cours : fournisseur B réel et frontières de la référence 3D.
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : « Continue », 2026-09-19.
Capacité : la référence 3D consomme le fournisseur spectral B du cœur ; ses frontières
sont éprouvées contre le chemin 2D reçu. Consommateur : banc et aperçu 3D, puis référence
pour la production GPU d’ADR-175. La scène interactive reste la prochaine intégration.
Critères avant code : échantillons MAC identiques au ponctuel B prolongé, aucune allocation
après configuration, aucun échantillon partiel publié sur refus ; cas plan progressif et
éponge comparés au témoin 2D (hauteur < 3 mm S201, écarts et limites publiés).
Ne pas transformer cette équivalence en réception universelle de l’absorption oblique.
Le coût 2D est différé selon S293 ; ce lot prépare les entrées réelles de la porte B.

### Plan

- [x] **P1** — amorce, lecture ciblée et plan seuls ; copie unique, master propre.
- [x] **P2** — échantillonneur B réel sur MAC 3D, stockage réservé et publication atomique ;
  tests au ponctuel, erreurs et allocations.
- [x] **P3** — comparer houle progressive et éponge 3D au témoin 2D, transposition x/y ;
  fixtures et critères existants conservés, limites publiées.
- [>] **P4** — consommer B réel dans un aperçu 3D calculé, vérifier les images et la suite.
- [ ] **P5** — preuve et rituel REPRISE §6 : file, feuille de route, index, journal, jeton libre.

### Notes de reprise

B possède differential_local_extended (ADR-154) et differential_grid_extended (S276).
Le support Samples3 de S297 ne traite que des fonctions de banc infaillibles.
B est profond : le fond du domaine doit être suffisamment bas et l’atténuation publiée.
W prolongé au-dessus du plan moyen reste A286 ; ne pas prétendre le recevoir avec B seul.
P4 commence pendant les calculs P3, sans modifier le solveur ni les bancs en cours.

P4 : premier essai 64 modes trop lent en ponctuel ; arrêt après quelques images.
Réutiliser la grille S276 à chaque rangée y, mêmes bits, scratch réservé.
