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

Session : S193 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo 1.97.0 disponibles)
Objectif : S192-1, construire les conditions de surface **non linéaires** dispersives
sur la tranche x-z de S192 et les recevoir contre une référence de **Stokes**, avec
ordre en amplitude explicite et domaine de validité déclaré. Premier véhicule à la fois
non linéaire et dispersif (A217) ; aucune sélection de solveur δ, aucun seuil redemandé.

### Plan

- [x] **P1** — reprise, état réel (trois copies au commit de `master`), jeton et plan seuls.
- [x] **P2** — dérivation et protocole **avant tout code** : équations de Zakharov exactes,
  développement en amplitude de la vitesse verticale de surface, symboles verticaux
  discrets tirés du relèvement S192, échelle d'ordres M=1/2/3 et signature prédite de
  chacun, références Stokes (profil d'ordre 2, correction de fréquence d'ordre 3),
  convention de courant moyen, anti-repliement, réceptions chiffrées déclarées.
- [x] **P3a** — véhicule : support non linéaire (état spectral, réutilisation du
  relèvement, vitesse verticale à l'ordre M, RK4, projection de bande), tests propres
  dont **réduction exacte à S192 à M=1** et refus atomiques.
- [x] **P3b** — campagne : erreur de profil contre Stokes-2 en fonction de l'amplitude
  (pente déclarée), décalage de fréquence contre Stokes-3, profondeurs, raffinements
  espace/temps, contre-épreuves par l'échelle M, reproductibilité deux exécutions.
- [ ] **P4** — documenter résultats, limites et suite ; propager A50/B4/A216/A217, file
  plurielle, angles et leçons. ADR seulement si une décision de projet est prise.
- [ ] **P5** — rituel de fin (§6) : journal, angles, leçons, index/README/décomptes,
  jeton `libre`, trois copies avancées sans suppression non prouvée.

### Notes de reprise

Lu dans cette conversation : AGENTS.md, REPRISE.md entier, EN-COURS, dernier journal
(S192), SURFACE-LIBRE-2D-S192, file active S190 de QUESTIONS-OUVERTES, libellés des
18 invariants, `support/free_surface.rs` entier.

État hérité : S192 fournit un véhicule **linéaire** spectral (η, ψ en DFT horizontale,
relèvement tridiagonal en profondeur, symbole `G_h`, Verlet). Erreurs Airy fines
≤1,732796 % ; empreinte `0x4fc690d4ac035bf7`. Seuil B4 = 2 % fixé (ADR-120), **jamais
redemandé**. ADR-112 interdit de conclure la bascule d'une superposition linéaire.

Thèse de la session, déclarée avant mesure : une troncature en amplitude à l'ordre M
donne trois signatures **distinctes et falsifiables** — M=1 ne produit aucune harmonique
liée ni décalage de fréquence, M=2 produit l'harmonique liée mais pas la bonne
fréquence, M=3 produit les deux. C'est l'échelle qui reçoit le véhicule, pas un seul
chiffre. Détails dans le document de protocole écrit en P2.

P2 : protocole écrit dans SURFACE-LIBRE-NL-S193. Choix arrêtés avant code —
équations de Zakharov exactes dérivées sur place ; développement HOS/Craig-Sulem
avec symboles verticaux A=G_h et B=k², dérivées supérieures algébriques ;
**représentation spectrale en bande Q avec convolution tronquée**, ce qui supprime
la question du repliement au lieu de la calibrer ; horizontal exact (µ=k_q²dz²,
changement assumé vis-à-vis de S192) pour ne laisser que K comme axe spatial ; RK4.
Trouvé en dérivant, non anticipé : (1) le peu profond de S192 **n'a pas d'oracle**
de Stokes à amplitude utile, borne d'Ursell U=aL²/h³ ; (2) la fréquence de Stokes
d'ordre 3 en profondeur finie **dépend d'une convention de courant moyen**, donc
seul le cas profond peut servir d'oracle de fréquence ; (3) volume et énergie ne
sont plus conservés exactement — la dérive est un diagnostic de troncature.
Ces trois points sont des candidats d'angle mort à instruire en P4.

P3a : support/nl_surface.rs + coquille d'exemple. Quatre tests passent en debug
et release, build sans avertissement. L'oracle du test M=1 a dû être **corrigé
avant d'être écrit** : RK4 n'intègre pas exactement l'oscillateur, l'oracle exact
est la puissance fermée de son amplification (precision datee dans le document).
Controle de vie, h=8, ka=0,05, 1 periode, K=64 : |eta2| **exactement nul** a M=1,
1,579404e-3 a M=2, 1,585220e-3 a M=3 ; energie -8,0e-11 relatif ; volume 1,9e-19 ;
residu de relevement 2,220e-16. Le volume derive **bien moins** que l'ordre de
troncature ne l'exigeait : prediction du protocole conservatrice, a dire en P4.
Parti pris retenu : etat spectral en bande, convolution tronquee, psi_0 jauge a zero
(prouve inerte par un test), symbole horizontal exact donc K seul axe spatial.

P3b : campagne executee, reception=true, empreinte 0x41fc3b13793bee10, deux
executions release identiques, quatre tests debug/release. L'echelle M=1/2/3 rend
les trois signatures predites. M=2 donne **exactement la moitie** du decalage de
Stokes en profond et **0,663** en intermediaire : la part captee depend du regime.
Volume conserve **exactement** a M=2 (annulation algebrique des deux termes d'ordre
deux au mode nul) : prediction du protocole fausse, mecanisme publie. Ursell mesure
comme une falaise, et le vehicule est juste en faible profondeur dans le domaine.
Le contre-epreuve d'amplitude negligeable a trouve le seul defaut : condition
initiale batie sur omega0 du continu au lieu de omega_d, mode elliptique, biais
1,1e-7 (rapport 16,1 entre h=8 et h=2, = rapport des ellipticites). Corrige ;
biais residuel -5,0730e-10 = erreur de phase RK4 predite analytiquement.
Candidats P4 : angles morts (oracle absent en faible profondeur ; convention de
courant moyen ; condition initiale du continu sur modele semi-discret) et lecons.

