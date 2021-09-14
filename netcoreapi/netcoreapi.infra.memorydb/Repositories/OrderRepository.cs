using Microsoft.EntityFrameworkCore;
using netcoreapi.domain.entities;
using netcoreapi.domain.Interfaces.Repositories;
using netcoreapi.infra.memorydb.Db;
using System.Collections.Generic;
using System.Linq;

namespace netcoreapi.infra.memorydb.Repositories
{
    public class OrderRepository : IOrderRepository
    {
        private readonly ApiContext _context;

        public OrderRepository(IDbContextFactory<ApiContext> dbContextFactory)
        {
            _context = dbContextFactory.CreateDbContext();
        }

        public void Add(Order order)
        {
            _context.Orders.Add(order);
            _context.SaveChanges();
        }

        public void Delete(string id)
        {
            var orderToRemove = _context.Orders.Find(id);
            if (orderToRemove == null)
                return;

            _context.Orders.Remove(orderToRemove);
            _context.SaveChanges();
        }

        public ICollection<Order> GetAll()
        {
            return _context.Orders.ToList();
        }

        public Order GetById(string id)
        {
            return _context.Orders.Find(id);
        }

        public void Update(Order order)
        {
            _context.Update(order);
            _context.SaveChanges();
        }
    }
}
