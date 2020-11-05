from bs4 import BeautifulSoup
import re
import sys
import json

from settings import pokemons_html_folder, poke_list_file


def find_first_block(soup):
    return soup.find_all("div", class_="mw-parser-output")[0]

def find_origin(soup):
    h3 = [ s for s in soup.find_all("h3") if s and s.span and s.span.get("id") == "Origem_do_nome" ]
    origin = h3.findNext("p")
    return origin

def find_height(soup):
    print("TODO!")
    
def find_weight(soup):
    print("TODO!")
    
def find_category(soup):
    print("TODO!")

def find_type(soup):
    print("TODO!")


def main(file_):
    html = None
    with open(file_, "r") as fp:
        html = fp.read()
    soup = BeautifulSoup(html, 'html.parser')
    print(find_first_block(soup))

if __name__ == "__main__":
    main(sys.argv[1])
