import os

data_folder = "../../data"
poke_root_url = "https://pokemon.fandom.com/pt-br/wiki/"
poke_list_url = "https://pokemon.fandom.com/pt-br/wiki/Pok%C3%A9dex_Nacional"
poke_list_file = os.path.join(data_folder, "pokelist.json")
pokemons_html_folder = os.path.join(data_folder, "pokemons_pages")
poke_db = "pokemon_db.json"
scraping_delay_in_seconds = 10
