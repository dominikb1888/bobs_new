curl -s https://dominikboehler.de/bobs_new/calendar/ | grep "|" | awk -F \| '{print $2}' | tail -n 14 | cat -n
