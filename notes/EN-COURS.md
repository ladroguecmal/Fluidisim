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

Session : S429 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« On accepte ce petit surplus au critère, continue »* —
**C7e est reçu** à 2,07 ms au p99 (pas + bascule, B10 en bande étroite ; critère 2 ms) par décision de l'utilisateur ; la suite
déclarée : **C7d, relative à B** ([preuve](../docs/validation/APIC-CARTE-S416.md) §1.1 : *« la bande sur la production couplée
(ADR-198), le fond qui suit la vitesse propre de δ (BANDE-ETROITE-S413 §6.4) — reçu si la vague de Chen sous B : particules seulement
où δ se déforme ; rien sous une houle calme »*).

**Ce que la session trouve en entrant.** C7d n'est pas un portage : la production GPU ne porte pas encore le mode relatif d'ADR-198
(D1 : « travail daté ») ; A320 (une perturbation de δ croît sous houle raide, en mode relatif) est ouverte ; la bande d'APIC simule
l'eau totale, sans B. **Mais son critère de réception se mesure en référence dès maintenant** : le banc de la vague de Chen
(`apic3d_deferlement`) initialise l'eau par le champ linéaire de Stokes ; pris pour B et propagé, il donne `u − U_B = 0` au départ —
la vitesse propre de δ ne naît que là où l'écoulement quitte la houle progressive.

**Ce que la session fait.** (1) **C7e reçu** : consigné (preuve §20, registres, liste). (2) **La conception de C7d** (preuve §21) :
ce qu'elle demande, ses dépendances (le mode relatif sur la carte, A320), son découpage — **C7d-1** le critère relatif en référence
sur la vague de Chen ; **C7d-2** le même sur la carte ; **C7d-3** la bande dans la production couplée, après le mode relatif sur la
carte — chacun avec son « reçu si » écrit avant. (3) **C7d-1** : `ColumnsSwitch` reçoit un fond B analytique (houle linéaire : `a`,
`k`, `ω`, phase, niveau moyen) ; le critère de vitesse du fond (`floor_speed`, S415) porte alors sur `|u − U_B(x, t)|` ; éteint
par défaut (au bit sans lui).

**Critères de C7d-1, écrits avant.** Sur la vague de Chen (40 mailles par longueur d'onde, `ny` 4, maintien 0,3 s, fond 4, vitesse
0,2 m/s) : (a) **`ε` = 0,55** — la part des colonnes de la fenêtre en particules au retournement **sous 0,5** (vitesse absolue :
1,000 ; la forme seule : 0,50), le retournement **au même instant** que la forme seule (à un pas près) ; (b) **`ε` = 0,1, houle calme**
— **aucune colonne de la fenêtre en particules** après le premier pas, sur 2,5 τ ; (c) sans fond B, S415 au chiffre près ; (d) suite
du cœur, zéro avertissement. Les parois réfléchissent (le bassin n'est pas périodique) : la part hors fenêtre est publiée, pas jugée.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — C7e reçu : preuve §20, registres, liste.
- [x] **P3** — la conception de C7d : preuve §21.
- [ ] **P4** — C7d-1 : le fond B analytique dans `ColumnsSwitch`, le critère relatif ; essais du cœur.
- [ ] **P5** — C7d-1 mesuré sur la vague de Chen (`ε` 0,55 et 0,1) ; critères.
- [ ] **P6** — suite ; preuve ; registres.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2** — C7e reçu, consigné : preuve §20 (la décision et sa portée : 2,07 ms mesurés sur B10 en bande étroite, non sur la scène de la porte B ; les dispatchs indirects restent désignés), liste 4.19, file (`C7d`, §21).
- **P3** — la conception de C7d, preuve §21 : ce qui existe et manque (le mode relatif en référence seulement ; A320 ouverte, et la vague de Chen est une houle raide ; la bande simule l'eau totale ; les critères d'écoulement de S415 absents de la carte) ; **C7d-1** le critère relatif en référence sur la vague de Chen, **C7d-2** le même sur la carte, **C7d-3** la bande dans la production couplée, après sa propre conception (le mode relatif sur la carte, A320 d'abord) — chacun avec son « reçu si ».
