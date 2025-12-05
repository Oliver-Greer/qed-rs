import spacy
from typing import List

def get_token_info(paragraph: str) -> List[dict]:
    nlp = spacy.load("en_core_web_sm")
    sentences: List[dict] = []
    for token in nlp(paragraph):
        if not token.is_punct and not token.is_space:
            sentences.append(
                {"text": token.text,
                "lemma": token.lemma_,
                "pos": token.pos_
                }
            )

    return sentences