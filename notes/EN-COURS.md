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

```
Session          : S66
État             : terminée
Agent            : Codex (git et cargo disponibles)
Objectif         : Diagnostiquer le nouvel échec d’homogénéité (S65-1).
```

### Plan

- [x] **P1** — état réel, passation, jeton et plan seul.
- [x] **P2** — diagnostic à phases spatiales et sommation f64 sur les mêmes composantes ; fenêtres 144/576/1536 m, six graines, translation 3000 m.
- [x] **P3** — confronter les mesures, ajouter un témoin de causalité et documenter la portée ; aucun seuil modifié.
- [x] **P4** — rituel : journal, actions, angles/leçons si établis, index, corrections datées et jeton libre.

### Notes de reprise

Départ master 25d3a9e propre ; copie 29ef50 identique et propre. REPRISE et socle lus dans
cette conversation, dernière entrée S65 relue. Ordre prévu S65-1 puis S64-3 puis S64-2.
La fenêtre homogénéité vaut 48*3=144 m, distincte du Hs à 3072 m. À 3000 m, la fenêtre
1536 m reste dans le rayon 4096 m ; celle de 3072 m dépasserait ce rayon. Ne pas la copier.
S63-1 reste le blocage B2. Tolérance 15 % conservée ; aucun choix de graine pour rendre vert.

P2 : dix-huit couples de fenêtres mesurés, diagnostic release 9,46 s. Nominal ratio
production 1,397506641, référence spatiale f64 1,397506306 ; somme des variances par composante
ratio 0,979715316. Écart interférences : cross_p=0,006459304, cross_l=0,046176100 m².
Fenêtre nominale 144 m, 576 m -> 1,079318732, 1536 m -> 0,943614087. Aucune correction
production ; contrôler par un témoin monochromatique et conserver le verdict en P3.

P3 : test de causalité et témoin monochromatique verts ; 134 tests réussis, quatre ignorés.
Check inchangé. HOMOGENEITE-S66 contient les 18 ratios et la décomposition. Aucun chemin de
production ni seuil modifié ; commentaire du contrôle corrigé. S65-1 peut être close,
S66-1 portera la décision de séparer contrôle de précision et diagnostic statistique.

P4 : A188/L183 consignés, S65-1 close, S66-1 ouverte. Suite S67 S64-3. Aucun calcul en cours,
jeton libre, copie 29ef50 à avancer comme aux étapes précédentes.
