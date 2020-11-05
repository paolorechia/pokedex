import requests
import os

from settings import poke_list_url, data_folder

def main():
    response = requests.get(poke_url)
    response.raise_for_status()

    output_file = os.path.join(data_folder, "root_html.html")
    with open(output_file, "w") as fp:
        fp.write(response.text)


if __name__ == "__main__":
    print("Downloading root html")
    main()
