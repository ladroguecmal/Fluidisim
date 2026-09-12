#!/bin/sh
# S198 — recalcule l'état de vélocité du dépôt. Aucun état recopié (A185) : tout vient
# de `git` et du contenu, et se relit à chaque exécution.
#   sh outils/velocite.sh
# À défaut de shell, chaque bloc est une commande lisible et exécutable à la main.
set -e
cd "$(dirname "$0")/.."

echo "== Les quatre couches : derniere session ayant touche leur code d'execution =="
# `--follow` par fichier : un renommage est un changement de la couche, pas une disparition.
# S199 a paye cette lecon — le module s'appelait `volume.rs`, l'outil le classait en V, et
# apres renommage il le declarait « jamais avance ».
for lay in "B          :background"            "W          :impact|wake|pressure|radial|modal|spectral|composition|prepared|mixed|wave_"            "delta      :delta|shallow|dispersif|eponge|projection"            "V          :network|reseau|pipe|conduite"; do
  name=${lay%%:*}; pat=${lay#*:}
  # La bibliotheque seule : le harnais est de la machinerie de validation, pas la couche.
  files=$(ls code/water-core/src/*.rs 2>/dev/null | grep -v "/tests_" | grep -E "$pat" || true)
  n=0; last=""
  for f in $files; do
    n=$((n+1))
    s=$(git log --follow --format='%s' -- "$f" | grep -oE '^S[0-9]+' | sort -u -V | tail -1)
    if [ -n "$s" ]; then
      last=$(printf '%s
%s
' "$last" "$s" | grep -v '^$' | sort -u -V | tail -1)
    fi
  done
  echo "  $name modules=$n  derniere avancee=${last:-jamais}"
done

echo
echo "== Ou va le temps : lignes ajoutees par ere =="
echo "   ere        bibliotheque   bancs   documents   part systeme"
for r in "150 159" "160 169" "170 179" "180 189" "190 199"; do
  a=${r% *}; b=${r#* }
  pat=$(seq "$a" "$b" | sed 's/^/S/' | paste -sd'|' -)
  git log --numstat --pretty=format:"%s" --all \
    | awk -v pat="^($pat) P" -v A="$a" -v B="$b" '
        /^S[0-9]+ P/{ok=($0~pat); next}
        NF==3 && ok {
          if ($3 ~ /^code\/(water-core|water-harness)\/src\//) s+=$1;
          else if ($3 ~ /^code\/water-core\/examples\//) e+=$1;
          else if ($3 ~ /\.md$/) d+=$1 }
        END{ t=s+e+d; if(t==0) t=1;
             printf "   S%s-S%s   %8d  %7d   %9d   %6.1f %%\n", A, B, s, e, d, 100*s/t }'
done

echo
echo "== Le corpus s'entretient-il lui-meme ? =="
md=$(find docs notes -name '*.md' -exec cat {} + | wc -l)
rt=0
for f in $(ls code/water-core/src/*.rs code/water-harness/src/*.rs | grep -v "/tests_"); do
  t=$(grep -n "^#\[cfg(test)\]" "$f" | tail -1 | cut -d: -f1)
  n=$(grep -c "" "$f")
  rt=$((rt + ${t:-$((n+1))} - 1))
done
echo "  markdown=$md  code d'execution=$rt  ratio=$(awk -v a="$md" -v b="$rt" 'BEGIN{printf "%.1f", a/b}') lignes de prose par ligne de code"
# Le registre mêle lignes de tableau et puces ; compter les identifiants uniques,
# pas une seule présentation Markdown (236 puces pour 243 identifiants avant P4 S199).
angles=$(grep -E '^(\|[[:space:]]*|-[[:space:]]+)(\*\*)?A[0-9]+' docs/registres/ANGLES-MORTS.md | grep -oE 'A[0-9]+' | sort -u | wc -l)
echo "  angles morts=$angles  notes correctives=$(grep -rho '^\*\*Note S[0-9]*\|^> \*\*Correction' docs --include=*.md | wc -l)"

echo
echo "== Le chainage : combien de sessions ont pris le reliquat de la precedente =="
tot=0; chain=0
for i in $(seq 160 250); do
  grep -q "^## S$i " notes/JOURNAL.md 2>/dev/null || continue
  tot=$((tot+1))
  grep -q "S$((i-1))-1" notes/JOURNAL.md && chain=$((chain+1)) || true
done
echo "  $chain sur $tot sessions depuis S160"
