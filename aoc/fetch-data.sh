year=$1
day=$2

curl "https://adventofcode.com/$year/day/$day/input" \
  -H 'accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7' \
  -H 'cookie: session=53616c7465645f5f99e3e6fbae4dccf31d3eb0770bbd7e3e1ea7734937c999f28365edff3c9e773f2c05acd529809f8098775a43f6fa6c2b5f583d13d4961cb5' \
  -H 'referer: https://adventofcode.com/2024/day/1'
