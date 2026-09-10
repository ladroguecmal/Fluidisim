# S163 — Intégrer le résidu couplé en Saint-Venant

2026-09-10. Action S162-1, ADR-112. Instrument 1D, lit plat, eau strictement mouillée,
hors runtime. Aucun choix de solveur δ 3D ni seuil de bascule.

## 1. Équation et schéma déclarés avant construction

Référence : `Shallow1D`, Rusanov d'ordre un en espace, SSP-RK2 en temps, murs réfléchissants.
Ordre un retenu pour rendre le flux résiduel explicitement vérifiable, sans reconstruction ni
limiteur non linéaire supplémentaire. Le modèle est celui documenté en tête de `shallow.rs`.

État conservatif total `T=(h,q)` avec `q=hu` ; fond choisi `Q=(H,M)` ; résidu stocké
`d=(z,r)`, avec `h=H+z`, `q=M+r`. **r n'est pas une vitesse perturbative.**
Le flux physique est `F(h,q)=(q,q²/h+gh²/2)`. Pour `U=M/H`, notre dérivation donne :

```
ΔF(Q,d) = F(Q+d)-F(Q)
ΔF_h = r
ΔF_q = 2Ur - U²z + (r-Uz)²/(H+z) + gHz + gz²/2
```

À une interface de Rusanov, `a_T` et `a_Q` sont les maximums de `|q/h|+sqrt(gh)` des deux
états adjacents, évalués respectivement sur T et Q. La différence des flux **numériques** est :

```
ΔF_num = (ΔF_gauche+ΔF_droite)/2
          - [a_T saut(d) + (a_T-a_Q) saut(Q)]/2
```

Le dernier terme est un couplage **numérique**, absent de l'équation continue. L'omettre
ne reçoit plus le même schéma discret. Aucun calcul d'un total avancé puis soustrait : le
résidu possède ses tableaux, ses dérivées et ses deux étages RK2.

Deux fonds testés avec la même référence totale :

- **Fond évolué**, `Q_t=L(Q)` : `d_t=L(Q+d)-L(Q)`, calculé par divergence de ΔF_num.
- **Fond figé non uniforme**, `Q_t=0` : ajouter `L(Q)` à cette divergence. Cette source
  est l'analogue conservatif de `−S` de SPEC-004 §6.1. Même à résidu initial nul, elle doit
  faire évoluer le total : la bosse figée n'est pas une solution de Saint-Venant.

Murs : hauteur copiée, débit inversé, séparément pour Q et d ; pas imposé identique pour
les deux intégrations. Vérifier le Courant à chaque étage, la finitude et la profondeur positive,
sans saturation silencieuse. Le fond évolué est aussi reçu seul contre `Shallow1D`.

## 2. Réception annoncée

1. Vérifier ΔF_num contre une différence directe de flux sur des états mouillés variés,
   avec débit du fond et résidu positifs/négatifs ; vérifier d=0.
2. Réception temporelle : deux bosses régulières, fond et perturbation ; comparer hauteur
   **et débit** reconstruits à la référence totale à chaque pas, pas seulement au dernier.
   Seuil d'instrument **1e-10**, normalisé par l'amplitude initiale et par celle-ci fois
   `sqrt(gh0)` pour le débit. C'est une tolérance d'arrondi f64, pas un seuil physique.
3. Contre-épreuves distinctes : retirer `g(H-h0)z` en conservant le transport sur profondeur
   de repos ; retirer `(a_T-a_Q)saut(Q)` ; retirer la source d'un fond figé non uniforme.
   Chaque défaut doit faire échouer le même critère, avec un témoin complet qui passe.
4. Raffiner séparément pas et mailles ; publier l'écart de reconstruction et l'écart de
   discrétisation de la référence comme deux grandeurs différentes. Ne pas appeler un accord
   entre deux écritures du même schéma une validation physique.
5. Cas limites : résidu nul, fond uniforme, perturbation négative sans cellule sèche,
   réflexion aux murs. Conservation de masse sur domaine fermé ; momentum global non conservé
   attendu à cause des forces des murs.

Les amplitudes, dimensions et durées de la sonde sont des fixtures de réception déclarées dans
le code ; aucune valeur n'est un paramètre du système de production. S161 reste un diagnostic
distinct (ordre deux/HLL), ses coefficients ne sont pas comparables directement à cette campagne.
