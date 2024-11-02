import os, json, sys, magic # <- https://stackoverflow.com/a/24433682
from PIL import Image, UnidentifiedImageError
from urllib.parse import quote


with open("sources/configs/directory.json", "r") as config_file:
    config = json.load(config_file)


def is_file_allowed(file_name: str) -> bool:
    for file_type in config["allowed_file_types"]:
        if file_name.endswith(file_type):
            return True

    return False


def load_source_folder_documents(source_folder: str) -> list[str]:
    global config
    documents = []

    if config["iterrate_through_folders"]:
        for root, _, files in os.walk(source_folder):
            for file in files:
                if is_file_allowed(file):
                    documents.append(os.path.join(root, file))

    else:
        dir_items = os.listdir(source_folder)
        for dir_item in dir_items:
            if os.path.isfile(os.path.join(source_folder, dir_item)):
                if is_file_allowed(dir_item):
                    documents.append(f"{source_folder}/{dir_item}")


    return documents


def load_documents() -> list[str]:
    global config
    documents_paths = []

    for source_folder in config["source_folders"]:
        source_documents = load_source_folder_documents(source_folder)

        if source_documents:
            for source_doc in source_documents:
                documents_paths.append(source_doc)

    return documents_paths

def process_documents(documents_paths: list[str]) -> list[dict]:
    documents_data = []

    for document_path in documents_paths:
        document_metadata = {}

        document_metadata["url"] = f"file:///{quote(document_path.lstrip('/'), safe=':/')}"
        document_metadata["title"] = os.path.basename(document_path)
        document_metadata["creation"] = int(os.path.getmtime(document_path) * 1000)

        document_mimetype = magic.from_file(document_path, mime=True)

        # @TODO if these types get more than 3.. make seperate functions to keep everything clean
        if document_mimetype.startswith("image"):
            try:
                image_document = Image.open(document_path)
            except UnidentifiedImageError:
                continue

            document_metadata["type"] = "image"
            document_metadata["width"] = image_document.width
            document_metadata["height"] = image_document.height


        documents_data.append(document_metadata)

    return documents_data

for line in sys.stdin:
        # strip off EOF
        request = json.loads(line.rstrip())

        result = {}
        error = None

        method = request['method']
        if method == "initialize/initialize":
            print(json.dumps({
                'id': request['id'],
                'result': result,
                'error': error,
                'version': "ed19eeb5298ecc9881cbb729fa427abb3ab36c40",
                'capabilities':  ["media/source"]
            }))

        else :
            result = process_documents(load_documents())

            print(json.dumps({
                'id': request['id'],
                'result': result,
                'error': error,
            }))
