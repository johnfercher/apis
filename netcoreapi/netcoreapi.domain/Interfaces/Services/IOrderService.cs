using netcoreapi.domain.entities;
using System.Collections.Generic;

namespace netcoreapi.domain.Interfaces.Services
{
    public interface IOrderService
    {
        public Order Add(Order order);
        public Order Update(string id, Order order);
        public void Delete(string id);
        public Order GetById(string id);
        public ICollection<Order> GetAll();
    }
}
