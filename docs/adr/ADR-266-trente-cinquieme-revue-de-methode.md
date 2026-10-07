# ADR-266 — Trente-cinquième revue de méthode (S651–S655) : les erreurs relevées, les contrôles du plan

- **Statut : actée**, S656, 2026-10-07 ; [ADR-222](ADR-222-la-methode-se-revise-elle-meme.md) D4 ; la précédente,
  [ADR-265](ADR-265-trente-quatrieme-revue-de-methode.md). **Sur la demande de l'utilisateur** : *« Les erreurs que tu réalises viennent
  d'où ? »*, puis *« Corrige et apprend de tes erreurs »*.

## 1. Les erreurs, rangées

| famille | sessions | coût |
|---|---|---|
| **une cause attribuée avant d'être isolée** | S639 (l'escalier au lieu du film du rivage), S653 (la force au lieu du couplage explicite) | une session de remède perdue (S640), une fausse piste (S654 l'a corrigée) |
| **un critère sur un instrument ou un comparant non éprouvé** | S644 (la remontée lue par les étiquettes), S652 (une traînée jugée sur un pic de choc ; l'instrument à 1,38 × Archimède, rattrapé), S655 (une sonde restée à la position de départ du corps) | trois critères manqués par l'instrument plus que par la physique |
| **un nombre écrit de mémoire** | S642 (31 modules au lieu de 43), S643 (un décompte de verdicts) | rattrapés par script |
| **un ADR lu sans ceux qui le nomment** | S609 (le seuil 0,35·Hs rétabli contre ADR-111/112) | un point faux pendant trente sessions |
| **un piège connu du domaine sous-estimé** | la ligne de contact en eau mince (S640, S644), la masse ajoutée d'un corps léger en couplage explicite (S653) | deux critères manqués |
| des glissements d'outil | S646 (le plan committé avec le travail), S647 (un shell rejeté) | rattrapés par le rituel |

**Ce que la relecture montre.** Les protections existaient déjà presque toutes dans [METHODE](../../notes/METHODE.md) : le témoin (L136,
L354, ADR-259 D1), l'instrument éprouvé (ADR-233 D1, ADR-263 D2), les nombres assertés (ADR-253 D1, ADR-263 D1), les ADR qui nomment
(ADR-259 D2). Elles n'ont pas été **appliquées au moment du plan**. Trente-six lignes ne se chargent pas d'elles-mêmes, et une hypothèse
plausible, prise vite, devient une certitude. Un contexte résumé y ajoute des détails perdus puis mal reconstruits.

## 2. Décisions

**D1 — Chaque plan porte un bloc « Contrôles du plan », cinq lignes** :

1. **témoin** : les causes candidates de ce qu'on va juger, et le montage qui en supprime une, s'il faut attribuer ;
2. **instrument** : le lecteur et le comparant, éprouvés sur un cas de réponse connue avant de juger ;
3. **calcul** : les nombres du plan calculés et assertés par son script ;
4. **ADR** : les décisions appliquées, et celles qui les nomment, lues ;
5. **pièges** : les pièges connus du domaine nommés (ligne de contact, masse ajoutée, CFL, égalités f32/f64, réflexions de bord…).

« Sans objet » se dit ; une ligne ne s'omet pas.

**D2 — Le rituel le vérifie.** À partir de S657, `rituel.py fin` refuse une session dont le plan n'a pas le bloc ou l'un des cinq mots.
Éprouvé en S656 sur trois textes : un plan complet est accepté, un plan sans bloc est refusé, un plan sans « pièges » est refusé.

**D3 — La mémoire.** Les six familles et les cinq contrôles sont aussi dans la mémoire persistante de l'assistant, relue à chaque nouvelle
conversation.

## 3. La prochaine revue

S661.
