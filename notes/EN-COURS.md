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

Session : S152 — terminée
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S151-1, exécuter un volet B2 à60s et publier une décision de domaine/profil, explicitement partielle.

### Plan

- [x] **P1** — état réel, dossier B2, protocole annoncé, jeton et plan seul.
- [x] **P2** — instrument B2 impact : lambda source2/3/4/5/6m, rayon80m, horizon60s ; admissibilité N64/128/256, erreur indépendante et contrôles d'oracle.
- [x] **P3** — traiter les refus de couverture si nécessaire par profil explicite ; coûts locaux répétés, reconstruction à30s, décision ADR et rapport.
- [x] **P4** — tests et rituel : journal, angles/leçons/actions, index/décomptes, passation et jeton rendu.

### Notes de reprise

Départ masterab3fee5, copies propres.292 tests/cinq ignorés,104 ADR.
B2 partiel : pas de GPU/champ2D/Boussinesq, pression bornée16s, ni D1 multiplateforme.
Comparer des résolutions du même candidat ne sélectionne pas sa technologie.
La longueur de source n'est pas lambda_cut. Erreur normalisée des sept composantes
<=1e-4 (S126) ; oracle radial512/1024 et angulaire1024/2048, variation<=1e-6.
Points centre et16 rayons irréguliers jusqu'à80m, temps0/1/10/30/45/60s, grille de réception
échantillonnée. Coût : cinq répétitions, p50/p99 locaux et dispersion ; reconstruction
WLIV à30s séparée de l'évaluation. Aucun nouveau seuil de célérité ad hoc.
Si profils existants refusent la couverture, essayer N512 explicitement par ADR,
sans affaiblir le garde de résolution. B2 global et lambda_cut resteront ouverts.
P2 : N64/128 refusent tout ; N256 accepte seulement lambda5/6 àR80/t60. Oracle radial max1,3651e-9, angulaire3,90e-16 ; erreurs N2565,385e-7 et4,110e-7. Profil N512 nécessaire à essayer, garde inchangé.

P3 : N512 reçu sur cinq fixtures, hashes debug/release identiques ; coûts et restauration30s dans BANC-B2-S152. ADR-105, défaut64 inchangé.

P4 :293 tests/cinq ignorés (200+93), zéro échec. Profil512 également release.105 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas, deux bancs partiels. Rituel exécuté et jeton rendu.
