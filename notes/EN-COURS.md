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

Session : S218 — terminée
Agent : Codex (fichiers, git et cargo disponibles)
Objectif : file J1/W, construire une borne locale de pente avec reste spatial démontré,
puis mesurer son resserrement et son coût sur les sillages S217.

### Entrée et état réel

Utilisateur : « Continue ». Master et trois copies propres à af1212b ; jeton libre,
maillons 1. Lectures de reprise S217 conservées dans cette conversation, état revérifié.
Travail dans la copie principale ; aucune dépendance ni copie nouvelle.

### Thèse avant construction

Sur un rectangle centré en c de demi-côtés hx,hy, la variation de pente d'un mode est
bornée par |eta_k| |k| (|kx|hx+|ky|hy), via sa Hessienne. On peut aussi la borner par
2|eta_k||k|. Donc norme(pente(c)) + somme des restes borne tout le rectangle.
L'intersection avec le majorant directionnel existant garde le meilleur des deux.
Le maximum des bornes sur une partition complète borne l'emprise, contrairement au
maximum des seuls échantillons. La sûreté algébrique et l'arrondi f32 restent distingués.
Ne pas modifier les bits du champ ni les anciens contrats d'admission sans réception.

### Plan

- [x] **P1** — jeton, thèse et plan seuls.
- [x] **P2** — lire les chemins réels, préciser la preuve et le contrat ; ADR si adoption justifiée.
- [x] **P3** — construire l'annonce locale dans le cœur, refus et contre-épreuves (centre trompeur compris).
- [x] **P4** — recevoir sur une partition des emprises S217 ; coût complet et resserrement, sans promesse de budget GPU.
- [x] **P5** — rituel §6, journal, file plurielle, index, reprise, jeton libre et copies synchronisées.

### Notes de reprise

Point d'entrée : spectral_pressure::Field et bound_pressure::Prepared ; les 356 tests
workspace release et deux tests exemple S217 sont les reçus précédents.

P2 : ADR-135 actée ; le reste porte les phases arrondies réelles (monotonie des produits aux bornes), avec réserve f32 distincte. API locale uniquement, aucune migration des admissions.

P3 : annonce locale construite, quatre tests ciblés debug passent. Refus contexte/temps/domaine/non-fini, zéro, centre trompeur, phases à4000m, couverture multidirectionnelle. Aucun calcul historique modifié. Battement P2 avait été écrit14:59 après horloge14:57 : erreur de recopie corrigée au prochain battement mesuré.

P4 en cours : premier passage complet (coût perturbé par compilation/tests simultanés,
ne pas en tirer un temps nominal). Gains à0,5m : base1,025625/lente1,224415/longue1,156292,
base tardive1,343845 hors durée. Toutes les bornes >= références S217. Deux passages
isolés de la base prévus pour coût. Correction P4 : refuser la norme qui sous-passe
à zéro ; quatre tests ciblés debug/release passent. Workspace release360/cinq ignorés
reçu avant cette dernière protection ; seuls ciblés rejoués ensuite, champs historiques intacts.
P4 reçue : passages isolés base 27,546–28,209s à0,5m ; valeurs imprimées identiques. Reçus et limites dans BORNE-LOCALE-S218. Suite adaptative portée à P5.

P5 : journal, A258/L297, index et file active actualisés. Prochaine S219 : partition adaptative à travail plafonné ; maillons0, jeton libre. Copies à avancer sur le commit de clôture après constat de propreté. Aucun travail de construction en attente dans S218.
