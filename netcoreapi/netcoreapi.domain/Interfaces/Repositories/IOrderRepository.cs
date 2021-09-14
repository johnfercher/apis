using netcoreapi.domain.entities;
using System.Collections.Generic;

namespace netcoreapi.domain.Interfaces.Repositories
{
    public interface IOrderRepository
    {
        public void Add(Order order);
        public void Update(Order order);
        public void Delete(string id);
        public Order GetById(string id);
        public ICollection<Order> GetAll();
        
    }
}
