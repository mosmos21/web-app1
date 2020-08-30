Web App1
====

```sh
# サーバー側をビルド 
gcloud builds submit --project private-develop --tag gcr.io/private-develop/echo-app --timeout=30000s app/

# フロント側をビルド
gcloud builds submit --project private-develop --tag gcr.io/private-develop/echo-nginx --timeout=30000s frontend/

# Cloud Run にデプロイ
gcloud run deploy --image gcr.io/private-develop/echo-app
gcloud run deploy --image gcr.io/private-develop/echo-nginx
```
