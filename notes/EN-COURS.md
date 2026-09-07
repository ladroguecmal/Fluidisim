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
Session          : S59
État             : en cours
Agent            : Claude Code (Opus 5 ; git et cargo disponibles)
Objectif         : S57-1 — découpage du calcul, borne du mode à 89600, mesure 89600/179200.
```

### Plan

- [x] **P1** — passation, jeton, plan seul.
- [x] **P2** — éprouver le découpage que S56 prescrit : mesurer s'il change le champ, avant de l'implémenter.
- [x] **P3** — implémenter le découpage retenu et relever la borne du mode à 89600 ; tests d'identité, d'admission et de refus.
- [ ] **P4** — mesurer 89600/179200 selon le critère **inchangé** ; consigner erreurs, écart, temps.
- [ ] **P5** — rapport MESURES-C22-S59 : admission de 12800, verdict des quatre fenêtres, apport au dossier A179 sans trancher S57-2.
- [ ] **P6** — rituel : journal, angles, leçons, actions, index, décomptes, passation, jeton, **fusion dans master**.

### Notes de reprise

Départ ae93fd0, master et worktree confondus. Attendu de S57 : admission de 12800 prévue avec
20 à 30 % de marge selon l'exposant retenu, coût projeté 1147 s (19 min 07 s). **Une admission
ne vaut pas un verdict** : la fenêtre 800–12800 n'aurait alors que trois ordres, et la stabilité
n'a jamais été établie sur 1,961 / 2,012.

**Réserve à lever d'abord, et elle est sérieuse.** REFERENCE-C22-S56 §5 prescrit, au-delà du
quart d'heure, « un découpage de calcul en tranches temporelles gardées en mémoire ». Or
`avancer_jusqu_a(t_fin, cfl)` choisit `dt = dt_cfl.min(t_fin − t)` : **découper l'intégration en
deux appels insère un pas tronqué au point de coupure**, qui n'existe pas dans le calcul
monolithique. La séquence de pas change, donc le champ change, donc toute comparaison avec
S48, S49, S56 et S57 tombe. Le remède prescrit invaliderait la mesure qu'il doit rendre
possible. À **mesurer** avant de conclure (L75), pas à supposer.

Voie envisagée si la mesure confirme : découpage d'**observation** — même boucle, mêmes `dt`,
même séquence exacte, avec un rendu de progression tous les N pas — et identité **bit à bit**
vérifiée par un test contre le chemin monolithique.

Interdits inchangés : pas de champ sur disque (I-17), pas d'extrapolation de Richardson du
champ, aucun changement de CFL, d'amplitude, de temps final ou d'initialisation, pas
d'assouplissement du filtre ×30 ni du test de stabilité. **Ne pas traiter S57-2 ici** : cette
session mesure sous le critère actuel ; discuter le critère est un autre travail, par ADR.

P2 : la réserve est confirmée par la mesure. le_decoupage_temporel_n_est_pas_neutre passe :
découper 0-1 s en quatre appels change le champ bit à bit, parce que chaque borne insère un pas
tronqué par min(t_fin - t). Le remède prescrit par REFERENCE-C22-S56 par.5 aurait invalidé
la comparaison avec S48, S49, S56, S57. Retenu à la place : découpage d observation.
avancer_jusqu_a_observe porte désormais la seule boucle, avancer_jusqu_a n en est qu un appel
avec observateur vide — identité structurelle, pas seulement testée.
Le second test a d abord échoué en trouvant une vraie faute de sa propre écriture : cadence 50
pour 35 pas, observateur jamais appelé, champ pourtant identique — un témoin muet aurait passé
pour neutre. Compte de rendus désormais vérifié contre le nombre de pas. 127 tests, deux ignorés.

P3 : borne du mode portée de 76800 à 89600, décrite comme limite de campagne. Refus vérifiés :
0, 12800, 32000, 102400 (multiple de 12800, refusé par la borne seule) et usize::MAX ; 89601
refusé aussi. 127 tests réussis, deux ignorés ; hashs check inchangés. Mesure P4 lancée sans
test concurrent.
