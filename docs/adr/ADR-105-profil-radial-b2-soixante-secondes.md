# ADR-105 — Profil radial explicite pour le volet B2 à60s

Statut : acté, S152, 2026-09-10. Complète ADR-085 ; aucun profil existant modifié.

## Constat et décision

Le premier volet exécuté de B2 demande un impact de longueur caractéristique
2/3/4/5/6m dans un disque de rayon80m, jusqu'à60s. N64 et N128 refusent tous les
cas ; N256 ne couvre que5 et6m. Le contrôle de phase par intervalle spectral
refuse les autres : ce contrôle ne constitue pas une mesure de précision.

Ajouter **N512 explicitement**, en conservant la plage historique64..256 et
le défaut64. Ne pas admettre implicitement257..511. Le garde de phase, la portée
Bessel, les conditions d'eau profonde et de pente restent inchangés.

La réception indépendante f64 (radial512/1024, angles1024/2048) reçoit N512
sur les cinq fixtures. Erreur normalisée maximale1,342e-6, seuil commun1e-4 ;
variation d'oracle<=1,366e-9. Le facteur de pente S139 est vérifié à N512.
Résultats et coûts complets : BANC-B2-S152.

**Choix de profil pour ce domaine échantillonné** : N512 pour2/3/4m, N256 pour5/6m.
Un profil unique couvrant les cinq demande N512. Ce n'est pas une interpolation
certifiée entre les fixtures ni un changement automatique du choix hôte.
N512 coûte12384 octets par champ, N2566240 ; sur5/6m, doubler N coûte environ
le double en requête et n'améliore pas systématiquement l'erreur f32.
La restauration WLIV porte toujours289 octets pour une source ; les coefficients
sont reconstruits. Les pools du service doublent les champs et ont leur propre coût.

## Ce que B2 ne décide pas encore

Ce sont deux résolutions du même candidat CPU. Pas de concurrent GPU/Boussinesq
construit, de D1 multiplateforme ou de budget matériel cible reçu. Pas de choix
global de technologie, de paquets_W_max, de Kelvin, de bathymétrie ni de lambda_cut.
La longueur caractéristique de la source n'est **pas** lambda_cut.
Le seuil de reconstruction complète les contrôles locaux ; il ne remplace pas
la campagne iso-célérité de DOSSIER-B2 §5. La conservation énergétique globale
sur60s reste à mesurer ; une sortie finie à60s ne la démontre pas.

I-06 : stockage fixe explicite ; I-03 : preuve distante ouverte ; I-07/I-08 :
milieu injecté et phases existantes ; I-18 : pente512 reçue sur les fixtures.
Aucun invariant changé. B2 est désormais **partiellement exécuté**, et son verdict
global reste ouvert. Suite : énergie/transport à60s sur ce domaine, puis réception
sillage long ou alternative bornée, sans réécrire ce verdict en sélection finale.
