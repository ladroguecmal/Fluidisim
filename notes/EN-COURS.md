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

Session : S182 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S181-1/A50, recevoir le consommateur différentiel dans le cycle vivant.

### Plan

- [x] **P1** — état réel, jeton et plan seul.
- [x] **P2** — déclarer le protocole cycle/direct, refus et rejeu avant les tests.
- [ ] **P3** — recevoir actualisation, admission, renouvellement et rejeu ; corriger si nécessaire.
- [ ] **P4** — rituel de fin, journal/leçons/index/décomptes, reprise et copies.

### Notes de reprise

S182 : master3541390 propre, quatre copies alignées ;117 ADR/225 angles/262 leçons.
Suite S181-1. Les contrôleurs existent ; éprouver le consommateur S181 sur leurs
publications contre préparation directe et rejeu. Ne pas inventer un nouvel ADR si
le contrat reste inchangé. Aucun coût ou solveur reçu par ces comparaisons.

S181 : master a77ea78 propre, quatre copies alignées ;116 ADR/225 angles/262 leçons.
Le montage mixed_water possède déjà classify (contexte/temps/perte), BoundBackground
et les vues de journaux. Réutiliser ces contrôles plutôt qu'inventer une association
locale sans instant. Pression imposée déjà incluse ; source après somme des champs.
Poursuite BILAN-S145/S176. Aucune copie nouvelle ; notes antérieures conservées.

P3 S181 : quatre tests nouveaux debug/release ; workspace328/cinq ignorés, C18/C02
inchangés. Réutilisation classify et helper de pente partagé. WorldPos ancré à1e9m,
deux impacts et deux pressions ; termes croisés >1e-6 reçus à1e-7m/s². Fixture
corrigée : confirm reçoit l'époque0, pas le numéro de séquence. Pas de seuil déplacé.
P4 :117 ADR/225 angles/262 leçons ; L260/L262 appliquées, pas de nouvelle
leçon nécessaire. Suite S181-1 cycle vivant avec ce consommateur différentiel.

S180 : master3b7cae0 propre, quatre copies alignées ;115 ADR/225 angles/261 leçons.
Le champ spectral conserve déjà pression et vitesse modales dans Slot. Dériver
depuis phi_t=-g eta-P/rho ; ne pas oublier la pression imposée dans p_dyn profond.
Suite BILAN-S145/S176, code de bibliothèque ; notes antérieures conservées ci-dessous.

P3 S180 : cinq tests nouveaux debug/release, workspace324/cinq ignorés ; C02/C18
inchangés. Source forcée reçue dès la naissance, contre-épreuve sans gradient de
pression détectée. Slot64 octets (+16), chemins incrémental et reliaison reçus.
P4 : rituel, L262,116 ADR/225 angles/262 leçons ; suite S180-1 composition
différentielle B+impacts+pressions, contexte et instant communs.

S179 : master ae28d98 propre, quatre copies alignées ;114 ADR/225 angles/260 leçons.
P4 S179 :115 ADR/225 angles/261 leçons,18 invariants,6 SPEC,23 cas vérifiés.
Suite S180 : S179-1 pression forcée W ; rituel terminé, jeton libre.
P3 S179 : six tests nouveaux debug/release ; workspace319/cinq ignorés ; C18/C02
inchangés. Référence angulaire512/1024, centre et lot reçus ; mutation isotrope rejetée.
Différences finies àh0,002 échouent pression, h0,01/0,005 passent sans changer seuil.
Poursuite de construction B4 (BILAN-S145/S176). Conserver phases, valeurs et refus
historiques ; origine radiale régulière. Notes S178 ci-dessous conservées comme entrée.

Master 4314dc8 propre ; quatre copies alignées, branche historique archivée conservée.
113 ADR,225 angles,259 leçons,18 invariants,6 SPEC,23 cas. S177 :307 tests/cinq ignorés.
Source continue distincte du résidu discret. Conserver eval et phases historiques.
B linéaire profond seulement ; pression relative au plan moyen ADR113, rho explicite.
Dériver avant de coder ; pas de zéro pour un terme non démontré nul.
BILAN-S145 porté par construction B4 après B1/S63-1. Aucune copie créée.
P2 : ADR114 ; gradients et Laplacien représenté, source S à soustraire. cargo check reçu.
P3 : six nouveaux tests, quatorze tests différentiels reçus ; workspace313/cinq ignorés.
Contre-épreuve : advection neutralisée, échec 0 contre0,2578228700 m/s² ; original restauré.
C18/C02 reçus, hashs inchangés ; source mono-mode quadratique et croisements reçus.
P4 : ADR114/L260 ;114 ADR,225 angles,260 leçons,18 invariants,6 SPEC,23 cas.
Suite S179 : différentiel RadialImpact puis composition B+W. Jeton libre après clôture.
