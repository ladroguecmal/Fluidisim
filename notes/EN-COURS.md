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

Session : S283 — en cours
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : continuer la porte A, après correction de mesure S282.
Objectif : premier redimensionnement spatial consommable de δ, avec transfert contrôlé de son
état et réception de ses limites ; ne pas confondre un changement de masque avec un gain de calcul.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — contrats et code du domaine : choisir un redimensionnement borné, déclarer ses
  critères, construire et tester le transfert sans allocation ; découper avant quinze minutes.
- [x] **P3** — consommation réelle par l'hôte ou le pas couplé, coût/continuité et refus ;
  suites de validation proportionnées, limites explicitement conservées.
- [>] **P4** — rituel §6 : journal, file, feuille de route, invariants et jeton libre.

### Notes de reprise

master dcaa97f propre ; une seule copie. Branche historique archivée vérifiée S282.
Le périmètre complet reste obligatoire ; V1 proposée S281 non redéfinie ici.
P2 précisé : transfert vers volume préalloué plus étroit, même dx/nz/milieu et fond plat.
Conserver hauteur/vitesses/reste d'arrondi dans l'intérieur ; smoothstep dans une bande fournie,
pression invalidée pour le nouveau domaine ; refus atomiques et aucune allocation. P3 : Live
128→64 colonnes, même horloge/fond aux coordonnées mondiales, publication de la vraie emprise.
Mesurer coût du transfert et des pas large/étroit ; continuité au centre, perte aux bords publiée.
Ce lot ne reçoit ni choix automatique non focal, ni agrandissement, ni I-12 perceptif.
P2 reçu : transfert + vrai pas couplé, zéro allocation au compteur global, centre au bit,
faces normales aux nouveaux bords nulles, refus atomiques (bornes et largeur).

P3 critère avant mesure : comparer les hauteurs publiées sur la bande au moment du transfert
contre 3 mm (tolérance d'image S201) ; publier tout dépassement, sans réception visuelle déduite.
Comparer aussi le centre |x|<16 m pendant les 128 pas suivants au témoin large. Pas de seuil de
vitesse pour faire passer la mesure : publier médiane et p99, transfert inclus séparément.
P3 reçu : Live/Layer et commande N, un rétrécissement manuel borné en hauteur, pas automatique.
Essai brut : 33,447 mm de saut, 21,255 mm de dérive centrale sur 2,048 s ; refus de la réception
sans couture. Garde Hermite + fondus + marge de Lipschitz ajouté : borne 33,729 mm, refus atomique
à 1,024 s ; admission vérifiée à 0,016 s. dx/64 trop conservateur au cas précoce, dx/256 retenu,
tolérance 3 mm inchangée. Sur secteur : 25,5464→12,3784 ms médian, 32,9817→15,8659 ms p99 ;
transfert + garde 3,0283 ms. Diagnostic forcé isolé, aucune réception I-05/visuelle.
Suites : 506 cœur/harnais + 33 viewer réussis, 19 ignorés au total, aucun échec.