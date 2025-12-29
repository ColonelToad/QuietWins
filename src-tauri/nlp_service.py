from fastapi import FastAPI, Request
from vaderSentiment.vaderSentiment import SentimentIntensityAnalyzer
import spacy

app = FastAPI()
nlp = spacy.load("en_core_web_sm")
analyzer = SentimentIntensityAnalyzer()

@app.post("/analyze_batch")
async def analyze_batch(request: Request):
    data = await request.json()
    texts = data["texts"]
    results = []
    for text in texts:
        sentiment = analyzer.polarity_scores(text)
        doc = nlp(text)
        entities = [{"text": ent.text, "label": ent.label_} for ent in doc.ents]
        results.append({"sentiment": sentiment, "entities": entities})
    return {"results": results}
