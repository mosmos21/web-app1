Web App1
====

```sh
# サーバー側をビルド 
gcloud builds submit --project private-develop --tag gcr.io/private-develop/echo-app --timeout=30000s app/

# フロント側をビルド
gcloud builds submit --project private-develop --tag gcr.io/private-develop/echo-nginx --timeout=30000s frontend/

# Cloud Run にデプロイ
gcloud beta run deploy --platform managed --region asia-northeast1 --image gcr.io/private-develop/echo-app echo-app
gcloud beta run deploy --platform managed --region asia-northeast1 --image gcr.io/private-develop/echo-nginx echo-nginx
```
