import requests
import os
import json
from time import sleep

from settings import (
        poke_root_url,
        data_folder,
        scraping_delay_in_seconds,
        poke_list_file,
        pokemons_html_folder
    )

def get_pokemons():
    with open(poke_list_file, "r") as fp:
        return json.load(fp)
    

def main():
    poke_list = get_pokemons()
    print(poke_list)

    if not os.path.exists(pokemons_html_folder):
        os.mkdir(pokemons_html_folder)

    for pokemon in poke_list:

        output_file = os.path.join(pokemons_html_folder, f"{pokemon}.html")

        print("Checking... {0}".format(pokemon))
        if os.path.exists(output_file):
            print("Done! Skipping.")
        if not os.path.exists(output_file):
            print("Downloading... {0}".format(pokemon))
            poke_url = poke_root_url + pokemon
            response = requests.get(poke_url)
            if response.status_code == 404:
                print(f"{pokemon} not found... :(")
                continue
            response.raise_for_status()
            
            sleep(scraping_delay_in_seconds)
            
            with open(output_file, "w") as fp:
                fp.write(response.text)
    
if __name__ == "__main__":
    print("Downloading pokemon list...")
    main()
