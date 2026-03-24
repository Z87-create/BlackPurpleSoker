import toml
import requests
from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel

with open("config.toml", "r", encoding="utf-8") as f:
    cfg = toml.load(f)

app = FastAPI(title="DeepSeek AI API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class AiQueryRequest(BaseModel):
    question: str
    role: str = "user"

@app.post("/api/ai/answer")
async def ai_answer(req: AiQueryRequest):
    api_key = cfg["ai"]["api_key"]
    api_url = cfg["ai"]["api_url"]

    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    }

    payload = {
        "model": "deepseek-chat",
        "messages": [
            {"role": "user", "content": req.question}
        ],
        "temperature": 0.7,
        "stream": False
    }

    try:
        resp = requests.post(api_url, headers=headers, json=payload, timeout=60)
        resp.raise_for_status()
        result = resp.json()
        answer = result["choices"][0]["message"]["content"]
        return {"answer": answer}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"DeepSeek error: {str(e)}")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="127.0.0.1", port=8000)