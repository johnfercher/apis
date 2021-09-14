using netcoreapi.domain.entities;
using netcoreapi.DTOs;
using netcoreapi.Mapper.Shared;

namespace netcoreapi.Mapper
{
    public class OrderToOrderResponseDto : BaseMapper<Order, OrderResponseDto>
    {
        public override OrderResponseDto Map(Order entry)
        {
            if (entry == null)
                return null;

            OrderResponseDto orderDto = new()
            {
                Id = entry.Id,
                Destiny = entry.Destiny,
                LabelCode = entry.LabelCode,
                Origin = entry.Origin
            };

            return orderDto;
        }
    }
}
