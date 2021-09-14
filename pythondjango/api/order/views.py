from rest_framework import viewsets
from django_filters import rest_framework as filters

from .models import Order
from .serializers import OrderSerializer


class OrderViewSet(viewsets.ModelViewSet):

    queryset = Order.objects.all()
    serializer_class = OrderSerializer
    filter_backends = (filters.DjangoFilterBackend,)
    filterset_fields = ('label', 'origin', 'destiny')