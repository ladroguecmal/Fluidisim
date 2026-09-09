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

Session : S127 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S126-1 / A204 — dimensionner ensemble rayon et horizon, puis recevoir
le transport d'énergie radial loin de la source sans dépasser N256.

### Plan

- [x] **P1** — état réel, jeton et plan seuls ; continuité de S126 sur master propre.
- [x] **P2** — dériver le domaine et construire une campagne : N256/R80/48 s visé,
      λ4 et source S126 inchangée. Refus des portées64/128 à N128/256 aux temps de groupe.
      Réemployer l'oracle S126 par module partagé, sans recopier les formules.
- [x] **P3** — recevoir surface et transport : sept composantes contre référence,
      densité physique positive intégrée en profondeur (formule S78) et rayon moyen.
      Raffiner spectre, angles et quadrature spatiale séparément ; rapport fidèle aux refus.
- [ ] **P4** — vérifications, rituel de fin, registres/actions/index/passation,
      jeton libre et commit propre.

### Notes de reprise

Départ760a9f0 sur master, aucune branche avancée, arbre propre ; aucune copie créée.
Règles et corpus déjà lus dans cette conversation. Le spectre et le milieu restent ceux
reçus en S126. Domaine visé80+cg_max*48≈164,8 m contre N*lambda/6=170,67 m disponibles.
Critères de banc AVANT mesure : sept composantes normalisées<=1e-4 (S126), oracle<=1e-6 ;
énergie totale fine à0,003 relatif et raffinement radial<0,002 (S78), spectral<1e-4.
Densité>=-1e-12 J/m² (S78). Transport demandé : majorité (>50 %) de l'énergie prescrite
hors32 m à48 s, quasi-absence (<0,003 E0) initialement ; seuils de banc, pas gameplay.
Le modèle profond de référence reçoit son bilan ; le candidat reçoit sa surface et sa
part potentielle. Ne pas annoncer son bilan cinétique complet sans accès à ses nœuds.
Comparer les sept sorties jusqu'à48 s, sans allonger le TTL source ni remettre les phases à zéro.

P2 : oracle extrait sans duplication vers examples/support/radial_reference.rs ; campagne
transport_extended_impact construite. N256/R80/T48 admis ; anciens couples aux temps de
R/cg_max refusent Resolution. Première campagne exécutée en15,94 s et reçue.
Énergie hors32 à48 s=0,999852 E0 ; rayon moyen53,4157 m ; erreur surface<=7,13e-7.
Résultats à publier et extraction S126 à vérifier en P3.
P3 : TRANSPORT-ETENDU-S127 publié. Deux bancs transport reçus15,94/15,22 s ; référence
S126 rejouée après extraction, valeurs conservées. Raffinements locaux de l'anneau reçus.
Surface max7,13e-7 ; potentiel candidat/ref max5,63e-7 E0 ; bilan total de référence seul.
S126-1 réalisée sur fixture, A204 traitée dans ce périmètre. S127-1 : cycle LiveWater B+W
N256/R80/horizon48 avec renouvellement, sauvegarde/reprise et coût. Aucun ADR nouveau.