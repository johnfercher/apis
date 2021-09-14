from django.contrib import admin
from django.urls import path, include

from rest_framework import routers
from order.views import OrderViewSet

api_router = routers.DefaultRouter()
api_router.register(r"orders", OrderViewSet)

urlpatterns = [
    path('admin/', admin.site.urls),
    path("api/", include(api_router.urls)),
]
