addpath(".");

close all
clear
clc
format long g

root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "processed", "aggregate", "Representation");
out_folder = fullfile(root_path, "processed");

out_file = fullfile(out_folder, "solution_representation.csv");

sizes = ["12", "16", "20", "24", "28", "32"];

algs = ["NSGAII","MOEAD","IBEA"];
algs_out = ["NSGA-II","MOEA/D","IBEA"];

ops = ["std","ca"];

fid = fopen(out_file, 'w');
fprintf(fid, "k,alg,std,ca\n");

alg_count = 0;

for i = 1:length(sizes)
    size = sizes(i);
    
    for alg = algs        
        alg_i = mod(alg_count, 3) + 1;
        alg_print = algs_out(alg_i);
        
        fprintf(fid, '%s,%s,', size, alg_print);
        alg_count = alg_count + 1;
        
        for j = 1:length(ops)
            op = ops(j);
            
            hv_file = fullfile(src_folder, size, op, alg, "aggregate.csv");
            
            if isfile(hv_file)
                agg = csvread(hv_file);
                mean = agg(2);
            else
                mean = 0;
            end
            
            fprintf(fid, '%f', mean);
            
            if j < length(ops)
                fprintf(fid, ',');
            end
        end
        
        fprintf(fid, '\n');
    end
    
    alg_i = mod(alg_count, 3) + 1;
    alg_print = algs(alg_i);
    alg_count = alg_count + 1;
    
    if i < length(sizes)
        fprintf(fid, '%s,%s,{},{}\n', size,alg_print);
    end
end

fclose(fid);