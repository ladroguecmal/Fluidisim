# ADR-060 — Un candidat radial borné, sans pavage du monde

- **Statut : ACTÉE**, S77, 2026-09-08, délégation technique.
- **Applique** ADR-059 ; produit `code/water-core/src/radial_impact.rs`.
- **Portée** : première quadrature de Hankel d’un impact isotrope profond. W3 reste partielle.

## Source et normalisation

Convention d’ADR-059 : eta = intégrale A(k) J0(kr) cos(omega t) k dk, omega²=gk.
Poser k0=2pi/lambda, a=k0/2, b=2k0, d=b-a et x=(k-a)/d. Spectre
A(k)=C x²(1-x)² sur [a,b], nul ailleurs. Ce choix de forme et de bande est **à calibrer B2**,
pas une loi physique de contact. L’événement apporte son énergie, pas la forme depuis le volume audio.

Parseval radial donne E=pi*rho*g intégrale A² k dk. Puis
intégrale_0^1 x^4(1-x)^4 dx=1/630 et sa moyenne de x vaut 1/2 par symétrie.
Donc intégrale A² k dk = C² d(a+d/2)/630 ; le code en déduit C.
Le spectre n’a pas de mode nul, ce qui donne un volume signé nul sur le plan continu complet.
Un disque tronqué ou une quadrature finie n’hérite pas automatiquement de cette égalité.

Milieu : g, rho, h et borne de pente injectés. h>pi/a impose le régime profond pour toute la
bande. La borne conservative de pente est la somme des poids positifs fois k, car |J1|≤1.
Elle doit respecter le seuil hôte, toujours à calibrer B2. Anisotropie refusée ; l’axe vertical
de la cause n’est pas une nouvelle hauteur de plan d’eau. La source est une condition initiale
à vitesse nulle, pas une simulation de la collision.

## Quadrature et domaine

N points médians, N de 64 à 256, tableau fixe sans allocation. J0 et J1 sont calculés comme
moyennes angulaires à 128 directions, avec PhaseQ32 et ses polynômes :
J0(x)=moyenne cos(x cos theta), J1(x)=moyenne cos theta sin(x cos theta).
Pas de libm dans le chemin de production. Racine carrée f32 pour rayon et dispersion.
Le domaine de cette approximation Bessel est limité à 0≤x≤64, contrôlé à chaque appel.
Cette limite et 128 directions sont des choix numériques testés, pas des paramètres gameplay.

L’hôte déclare rayon et durée maximum ; b*rayon≤64, rayon<4096, durée≤ttl, référentiel et cellule
cohérentes avec la source à l’évaluation. N<64 est refusé. Le contrôle de résolution demande
Delta_k*(rayon+c_g_max*durée)≤pi/2, avec c_g_max=0,5 sqrt(g/a). Il borne la variation de phase
par intervalle spectral, **pas l’erreur finale de quadrature**. Les durées de ce contrôle passent
en f64 seulement à la configuration ; l’évaluation temporelle emploie SimTime entier et PhaseQ32.

Les fréquences sont quantifiées Q32 et les amplitudes dérivées gardent omega nominal, comme
S75. Erreur temporelle longue durée et déterminisme croisé restent à recevoir. Les refus de
rayon ou de temps ne signifient ni eau nulle ni disparition d’énergie. Aucun TTL ne purge le journal.
Le support continu n’est pas périodique ; une somme finie ne promet pas une précision infinie.
Les domaines admis par ces contrôles sont des domaines de calcul, pas tous déjà validés physiquement.

## Résultats reproductibles S77

Commande : `cargo test --release --manifest-path code/Cargo.toml -p water-core radial_impact -- --nocapture`.
Scénario : lambda=4 m, énergie=0,01 J, g=9,81, rho=1025, profondeur 20 m, rayon 16 m,
durée 4 s, limite de pente 0,1. Paramètres d’essai uniquement.

- J0/J1 comparés à une quadrature f64 indépendante de 4096 directions décalées, aux arguments
  0 ; 0,01 ; 1 ; 2,4048256 ; 3,831706 ; 8 ; 16 ; 32 ; 64. Séries aux petits arguments
  et voisinage des premiers zéros contrôlés. Écart absolu exigé <4e-6, pas une preuve uniforme
  sur tous les arguments intermédiaires.
- N64 contre N128, rayons 0/1/4/8/16 m à 0/1/4 s : écart maximal d’élévation rapporté au pic
  initial **1,3e-7** (affichage arrondi). Tolérance de régression 1e-4 ; symétrie à un quart
  de tour exacte sur les axes testés.
- À 16 m, amplitude initiale/pic **0,00003731** : la copie exacte du carré S75 est absente.
  Cela ne prouve ni absence de toute oscillation lointaine ni localisation compacte.
- Énergie potentielle initiale dans le disque de rayon 16 m : rapport à l’énergie prescrite
  **1,00052365** sur 256 anneaux, **1,00012767** sur 512. Intégration de pi*rho*g*eta²*r dr,
  distincte de la normalisation spectrale. Tolérance 0,003 ; raffinement des anneaux améliore
  l’accord. Pas de bilan temporel total encore reçu.
- Volume signé du disque : **-0,00000037 m³**, puis **-0,00000115 m³**. Diagnostic de troncature,
  sans verdict de conservation globale. Une petite valeur sur un disque ne certifie pas la masse.

Les valeurs de source et seuils d’essai ne calibrent aucun profil du jeu. Quatre tests ciblés
incluent aussi refus du milieu, de l’anisotropie, de la pente, du rayon, du temps et de N.

## Suite et réception

S76-1 réalisée comme candidat exécutable borné ; W3 reste partielle. Prochaine étape S78 :
mesurer le bilan temporel physique et le déplacement radial, puis fixer un domaine de réception
qui contrôle ensemble troncature spatiale et quadrature. Le contrôle pi/2 seul ne le fournit pas.
Le coût de 128 directions par nœud est élevé : aucune promesse de budget temps réel ; optimisation
à partir du coût mesuré après réception physique. Journal→champ, WaterSample B+W, autorité réseau,
plateformes et budgets restent ouverts. Aucun choix final B2. I-03, I-07, I-08, I-14 relus, inchangés.

> **Actualisation S78 — 2026-09-08.** BILAN-RADIAL-S78 mesure l’énergie positive dans le temps
> et son transport. ADR-061 autorise l’intégration limitée sur le scénario reçu, sans élargir
> cette réception à tous les paramètres admis. Pas de compensation du déficit d’un disque fini.

> **Actualisation S82 — 2026-09-08.** ADR-064 remplace le noyau angulaire de production par
> une interpolation Hermite tabulée, domaine [0,64] inchangé. Référence angulaire conservée ;
> tests physiques repassés, nouveau comportement d’arrondi documenté. Domaine physique non élargi.

> **Actualisation S84 — 2026-09-08.** ADR-066 remplace la borne numérique au TTL :
> l'horizon se mesure depuis la naissance, indépendamment de la durée source. Renouvellement
> à résolution inchangée testé jusqu'à 16 s ; aucune purge TTL, rétention générale encore ouverte.

## Note corrective du 2026-09-10 (S137)

Cette décision renvoie la calibration **au banc B2**. C'est faux : B2 choisit la technologie de W
et `λ_cut`, et aucune de ses métriques ne mesure ce qu'un objet qui entre dans l'eau émet. La
calibration de la source relève de **B10**, dont le protocole — sphères et corps allongés à
Froude connu — fournit déjà les entrées, et auquel S137 ajoute deux métriques de source.
Voir [ADR-093](ADR-093-ou-se-calibre-la-source-d-impact.md).
