import os, json


with open("sources/configs/directory.json", "r") as config_file:
    config = json.load(config_file)


def is_file_allowed(file_name: str) -> bool:
    for file_type in config["allowed_file_types"]:
        if file_name.endswith(file_type):
            return True

    return False


def load_source_folder_documents(source_folder: str) -> list[str]:
    documents = []
    for root, _, files in os.walk(source_folder):
        for file in files:
            if is_file_allowed(file):
                documents.append(os.path.join(root, file))

    return documents


def load_documents() -> list[str]:
    global config
    final_documents = []

    for source_folder in config["source_folders"]:
        source_documents = load_source_folder_documents(source_folder)

        if source_documents:
            for source_doc in source_documents:
                final_documents.append(source_doc)

    return final_documents
