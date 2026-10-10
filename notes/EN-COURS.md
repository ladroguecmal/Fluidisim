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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S760 — **terminée**. En autonomie ; session longue. ADR-289 D3.3 : refaire les résultats importants avec la 3D corrigée
(ADR-294 : `Complete` consciente du fond, avec le compte cumulé d'énergie). **La question** : que deviennent les témoins du sélecteur
(SELECTEUR-DOMAINES-S732 §6), mesurés avec l'ancienne 3D trop haute (S734–S738) ?

**Ce qui est fait** :
- `ScenePlage.corrigee` : la 3D d'ADR-294 dans le témoin de S734 ;
- `ScenePlage.arret_mur` : le témoin s'arrête quand l'onde atteint le mur, à 5 mm du repos (ADR-286 D1, ADR-293 D4). S3 dépassait son mur ;
- `density_energy_removed` : l'énergie que le compte a retirée, cumulée (une lecture). Elle dit si la correction a joué.

En passant : `fermer.py` n'ajoutait pas `BOUSSOLE.md` au commit (S759 l'a laissé dehors) ; ajouté à ses chemins.

**Les critères, écrits avant** (ADR-290 D3, le repos d'abord ; ADR-293 D3, la bande de quantum) :
1. **le repos sur l'escalier de 1:3**, la pente de S4 (1 cm/s ; 3 mm) ;
2. **S4 sans déferlement**, `H/d` = 0,1 sur 1:3 (Synolakis : le déferlement au reflux dès 0,141) :
   - aucun retournement ;
   - la remontée par φ à 15 % de la loi, `2,831·√cot·(H/d)^(5/4)·d` = 0,1379 m, plus le quantum `dx/cot` = 8,3 mm ;
   - la remontée par les particules en seconde lecture ;
3. **S4**, `H/d` = 0,2 sur 1:3 (le déferlement au reflux permis, pas à la montée : 0,241) : la remontée à 15 % de 0,3280 m, plus 8,3 mm ;
   aucun retournement avant le maximum de la remontée ;
4. **S2** (Synolakis, `H/d` = 0,3 sur 1:19,85) : un retournement ; son instant et sa place rapportés contre l'ancienne 3D (S734 : 3,29 s) ;
5. **S3** (`H/d` = 0,5 sur 1:90), arrêté à l'arrivée au mur : le retournement rapporté, qu'il ait lieu ou non (la question de la scène) ;
6. **R43** (le montage de S755) : l'énergie retirée rapportée. Si elle est nulle, S755 reste vrai tel quel ; sinon, les grandeurs de S755
   sont comparées.

Le témoin est contrôlé comme l'objet (ADR-285 D1) : aucune sortie de particules, le front à plus de 1 m du mur.

**Contrôles du plan** (ADR-276, ADR-287, ADR-290, ADR-293)

- **témoin** : les témoins de S734–S738 (l'ancienne 3D) ; S755.
- **instrument** : le témoin de S734, son juge robuste et sa lecture par les particules (S737) ; l'énergie retirée (nouvelle ; sa seconde
  lecture est la comparaison au bit avec S755).
- **calcul** : le repos 2 min ; S4 deux fois 25 min ; S2 40 min ; S3 45 min ; R43 30 min.
- **ADR** : ADR-289 D3.3 ; ADR-285 D1 ; ADR-286 D1 ; ADR-293 D3, D4 ; ADR-294.
- **pièges** :
  - la loi de remontée vaut sans déferlement à la montée : S4 à 0,2 est sous le seuil (0,241), S4 à 0,1 sous les deux ;
  - le fond lisse ne tient pas le repos (S745) : l'escalier, et son repos vérifié d'abord ;
  - S3 : la durée bornée par l'arrivée au mur, non par une constante (S734 dépassait).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `corrigee`, `arret_mur`, `density_energy_removed` ; (1) le repos 1:3 ; (2) S4 à 0,1 ; (3) S4 à 0,2.
- [x] **P3** — (4) S2 ; (5) S3.
- [x] **P4** — (6) R43.
- [x] **P5** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** (2 696 s) — **les trois tenus** :
  - (1) le repos 1:3 : 9,0 mm/s ; 0,12 mm ;
  - (2) S4 à 0,1 : aucun retournement ; la remontée **0,1368 m contre 0,1379 m (−0,7 %)**, par φ et par les particules ; 0,78 J retirés ;
  - (3) S4 à 0,2 : la remontée **0,2835 m contre 0,3279 m (−13,6 %**, la bande ±17,5 %) à 3,83 s ; le retournement au reflux, 5,71 s
    (permis : 0,2 > 0,141) ; 0,58 J retirés.

  L'ancienne 3D : +37 % au moins (S735), +47 % (S734). Le témoin contrôlé : aucune sortie, le front à plus de 3 m du mur.
- **Pour la revue de S761** (les frictions de S756–S760) :
  - S758 : une règle réfutée en une session ; un examen statique de son signe (les déplacements vont dans les deux sens) l'aurait écartée
    avant le calcul (ADR-290 D3, « la cible vérifiée statiquement », appliquée à une règle de vitesse) ;
  - S759 : le bilan d'abord a choisi entre H1 et H2 avant tout remède : il a tenu ; la règle pas à pas a coûté un essai (le cliquet) ;
  - S759 : `fermer.py` laissait `BOUSSOLE.md` hors du commit (corrigé en S760) ;
  - S760 P1 : un script de plan en échec, suivi d'un `git add -A` sur une autre ligne : un commit sous un faux titre (corrigé par
    `--amend` avant la poussée). Le script et le commit sur une seule chaîne `&&`.
- **P3 fini** (4 467 s) : S2, le retournement à 4,14 s (x = 14,09 m), rien de retiré ; S3, le retournement à 6,84 s (16,41 m), l'onde au mur
  à 9,62 s, 2,16 J retirés.
- **P4 fini** (1 227 s) : R43, rien de retiré, **identique à S755** (3,0915 s ; 11,0875 m ; 0,3005 m).
