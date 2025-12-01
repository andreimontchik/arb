#Counts the number of arbs with unique profit amount.

file_name=$1
min_sol=$2
prior_slot=0
arb_slot=0

awk -F ',' -v min_sol="$min_sol" -v prior_slot="$prior_slot" -v arb_slot="$arb_slot" '

    $(NF-1) != "" && $20 != "" && $20 >= min_sol {

        if (arb_slot == 0){
            arb_slot = $3
        }

        if ($3 == prior_slot+1) {
            seen[arb_slot]++
        } else {
            count++
            arb_slot = $3
            seen[arb_slot] = 1
        }
        prior_slot = $3
    }
    END {  
        num_blocks = 2
        long = 0
        for (i in seen) {
            if (seen[i] > num_blocks) {
                long ++
            }
        }

        print FILENAME "|||" NR "|" count "|" long "|"
    }
' "$file_name"
