from fastapi import FastAPI, Request
from vaderSentiment.vaderSentiment import SentimentIntensityAnalyzer
import re

app = FastAPI()
analyzer = SentimentIntensityAnalyzer()

URL_RE = re.compile(r"https?://\S+|www\.\S+")
EMAIL_RE = re.compile(r"[\w\.-]+@[\w\.-]+\.[A-Za-z]{2,}")
CAPS_RE = re.compile(r"\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)\b")

def simple_tokenize(text: str):
    # very small, permissive tokenizer
    tokens = re.findall(r"\w+|'\w+|[.!?,;]", text)
    return tokens

def heuristic_entities(text: str):
    entities = []
    for m in URL_RE.finditer(text):
        entities.append({"text": m.group(0), "label": "URL"})
    for m in EMAIL_RE.finditer(text):
        entities.append({"text": m.group(0), "label": "EMAIL"})
    # simple proper-noun heuristic
    for m in CAPS_RE.finditer(text):
        token = m.group(1)
        if len(token) > 2:
            entities.append({"text": token, "label": "PROPER_NOUN"})
    return entities


@app.post("/analyze_batch")
async def analyze_batch(request: Request):
    data = await request.json()
    texts = data.get("texts", [])
    results = []
    for text in texts:
        sentiment = analyzer.polarity_scores(text)
        tokens = simple_tokenize(text)
        entities = heuristic_entities(text)
        results.append({"sentiment": sentiment, "tokens": tokens, "entities": entities})
    return {"results": results}


@app.get("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    import uvicorn
    # When frozen by PyInstaller, importing the module by name can fail.
    # Pass the `app` object directly to Uvicorn to avoid import-by-name errors.
    uvicorn.run(app, host="127.0.0.1", port=8000)
from fastapi import FastAPI, Request
from vaderSentiment.vaderSentiment import SentimentIntensityAnalyzer
import re

app = FastAPI()
analyzer = SentimentIntensityAnalyzer()

URL_RE = re.compile(r"https?://\S+|www\.\S+")
EMAIL_RE = re.compile(r"[\w\.-]+@[\w\.-]+")
CAPS_RE = re.compile(r"\b([A-Z][a-z]{2,}(?:\s+[A-Z][a-z]{2,})*)\b")

def simple_tokenize(text: str):
    # very small, permissive tokenizer
    tokens = re.findall(r"\w+|'\w+|[.!?,;]", text)
    return tokens

def heuristic_entities(text: str):
    entities = []
    for m in URL_RE.finditer(text):
        entities.append({"text": m.group(0), "label": "URL"})
    for m in EMAIL_RE.finditer(text):
        entities.append({"text": m.group(0), "label": "EMAIL"})
    # simple proper-noun heuristic
    for m in CAPS_RE.finditer(text):
        entities.append({"text": m.group(1), "label": "PROPER_NOUN"})
    return entities


@app.post("/analyze_batch")
async def analyze_batch(request: Request):
    data = await request.json()
    texts = data.get("texts", [])
    results = []
    for text in texts:
        sentiment = analyzer.polarity_scores(text)
        tokens = simple_tokenize(text)
        entities = heuristic_entities(text)
        results.append({"sentiment": sentiment, "tokens": tokens, "entities": entities})
    return {"results": results}


@app.get("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run("nlp_service:app", host="127.0.0.1", port=8000, reload=False)
