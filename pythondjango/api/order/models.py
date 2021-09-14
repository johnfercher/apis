from django.db import models


class Order(models.Model):
    id = models.UUIDField(primary_key=True)
    label = models.CharField(max_length=100)
    origin = models.CharField(max_length=100)
    destiny = models.CharField(max_length=100)