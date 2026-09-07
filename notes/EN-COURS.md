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
Session          : S57
État             : en cours
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : Exécuter le couple d'oracles 76800/153600 selon REFERENCE-C22-S56 (S56-1).
```

### Plan

- [x] **P1** — état réel, prise du jeton, plan seul.
- [x] **P2** — vérifier les tests et compiler en release avant toute mesure ; aucun changement de montage.
- [x] **P3** — exécuter `c22-shallow-fin 76800`, consigner erreurs, écart d'oracles, temps ; appliquer le filtre ×30 sans l'assouplir.
- [ ] **P4** — rédiger MESURES-C22-S57 : admission de 12800, verdict des quatre fenêtres, déplacement éventuel des sept anciennes grilles, budget révisé.
- [ ] **P5** — rituel : journal, angles, leçons, actions, index, décomptes, passation et jeton.

### Notes de reprise

Départ 13851c1, identique à master, dans le worktree claude/reprise-projet-29ef50. Les trois
autres copies sont en retard ou archivées ; aucune session concurrente. **La branche devra être
fusionnée dans master en fin de session** — c'est le mécanisme de fork L137/S39.

Attendu avant mesure : coût ~827 s (13 min 47 s) estimé par S56, sans garantie. Seuils extrapolés
pour l'admission de 12800 : 6,94346e-9 en n⁻², 7,77366e-9 avec l'exposant empirique 1,72145,
qui encadrent presque l'erreur mesurée en S56 (7,710700097e-9). Le modèle choisi changerait donc
le verdict d'admission : c'est précisément ce que cette mesure tranche.

Interdits rappelés par le protocole : pas de doublement automatique si 12800 est refusée,
pas de champ sur disque (I-17), pas d'extrapolation de Richardson du champ, aucun changement
de CFL, amplitude, temps final ou initialisation.

P2 : socle vérifié avant mesure, aucun fichier de code touché. 123 tests réussis
(38 cœur + 85 harnais), deux ignorés ; release compilée. check vert, hashs
0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés. Mesure P3 lancée sans test concurrent.

P3 : campagne 844,433 s (estimation S56 : 827,467 s, +2,05 %) ; oracles 165,833 et 672,560 s,
reste 6,040 s. Écart d oracles 2,709078717e-10, seuil 8,127236151e-9. Grille 12800 :
7,766762184e-9, soit 0,9556 fois le seuil — **encore refusée**, à 4,4 pour cent près.
Quatre familles sans verdict, sortie 0. Aucun code modifié.
