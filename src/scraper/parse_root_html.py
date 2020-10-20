from bs4 import BeautifulSoup
import re
import sys
import json
from io import StringIO

from settings import data_folder, poke_list_file


def main(file_):
    html = None
    with open(file_, "r") as fp:
        html = fp.read()

    pattern = '(?<=/pt-br/wiki/)[A-Z][a-z]*'
    soup = BeautifulSoup(html, 'html.parser')
    links = soup.find_all("a")
    poke_links = []
    for l in links:
        if l.get("href"):
            match = re.search(pattern, l.get("href"))
            if match and l.get("title"):
                if match.group() == l.get("title"):
                    poke_links.append(l)

    pokemons = sorted(list(set([p.get("title") for p in poke_links])))

    with open(poke_list_file, "w") as fp:
        json.dump(pokemons, fp)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit("Usage: {0} <html_file>".format(sys.argv[0]))
    main(sys.argv[1])
