using netcoreapi.domain.entities;
using netcoreapi.domain.Interfaces.Repositories;
using netcoreapi.domain.Interfaces.Services;
using System;
using System.Collections.Generic;

namespace netcoreapi.application.Services
{
    public class OrderService : IOrderService
    {
        private readonly IOrderRepository _orderRepository;

        public OrderService(IOrderRepository orderRepository)
        {
            _orderRepository = orderRepository;
        }

        public Order Add(Order order)
        {
            order.Id = Guid.NewGuid().ToString();
            _orderRepository.Add(order);
            return order;
        }

        public void Delete(string id)
        {
            _orderRepository.Delete(id);
        }

        public ICollection<Order> GetAll()
        {
            return _orderRepository.GetAll();
        }

        public Order GetById(string id)
        {
            return _orderRepository.GetById(id);
        }

        public Order Update(string id, Order order)
        {
            try
            {
                order.Id = id;
                _orderRepository.Update(order);
                return order;
            }
            catch(Exception e)
            {
                return null;
            }
            
        }
    }
}
