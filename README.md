Web App1
====

```sh
# サーバー側をビルド 
cd app
gcloud builds submit --project private-develop --tag gcr.io/private-develop/echo-app --timeout=30000s .

# フロント側をビルド
cd frontend
gcloud builds submit --project private-develop --tag gcr.io/private-develop/echo-nginx --timeout=30000s .

# Cloud Run にデプロイ
gcloud run deploy --image gcr.io/private-develop/echo-app
gcloud run deploy --image gcr.io/private-develop/echo-nginx
```
