using netcoreapi.domain.entities;
using netcoreapi.DTOs;
using netcoreapi.Mapper.Shared;

namespace netcoreapi.Mapper
{
    public class OrderRequestDtoToOrder : BaseMapper<OrderRequestDto, Order>
    {
        public override Order Map(OrderRequestDto entry)
        {
            if (entry == null)
                return null;

            Order order = new()
            {
                LabelCode = entry.LabelCode,
                Destiny = entry.Destiny,
                Origin = entry.Origin
            };

            return order;
        }
    }
}
